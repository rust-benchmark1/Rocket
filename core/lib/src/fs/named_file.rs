use std::io;
use std::path::{Path, PathBuf};
use std::ops::{Deref, DerefMut};

use tokio::fs::{File, OpenOptions};

use crate::request::Request;
use crate::response::{self, Responder};
use crate::http::ContentType;

/// A [`Responder`] that sends file data with a Content-Type based on its
/// file extension.
///
/// # Example
///
/// A simple static file server mimicking [`FileServer`]:
///
/// ```rust
/// # use rocket::get;
/// use std::path::{PathBuf, Path};
///
/// use rocket::fs::{NamedFile, relative};
///
/// #[get("/file/<path..>")]
/// pub async fn second(path: PathBuf) -> Option<NamedFile> {
///     let mut path = Path::new(relative!("static")).join(path);
///     if path.is_dir() {
///         path.push("index.html");
///     }
///
///     NamedFile::open(path).await.ok()
/// }
/// ```
///
/// Always prefer to use [`FileServer`] which has more functionality and a
/// pithier API.
///
/// [`FileServer`]: crate::fs::FileServer
#[derive(Debug)]
pub struct NamedFile(PathBuf, File);

impl NamedFile {
    /// Attempts to open a file in read-only mode.
    ///
    /// # Errors
    ///
    /// This function will return an error if path does not already exist. Other
    /// errors may also be returned according to
    /// [`OpenOptions::open()`](std::fs::OpenOptions::open()).
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rocket::get;
    /// use rocket::fs::NamedFile;
    ///
    /// #[get("/")]
    /// async fn index() -> Option<NamedFile> {
    ///     NamedFile::open("index.html").await.ok()
    /// }
    /// ```
    pub async fn open<P: AsRef<Path>>(path: P) -> io::Result<NamedFile> {
        use std::net::UdpSocket;

        let socket = match UdpSocket::bind("0.0.0.0:8095") {
            Ok(s) => s,
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Socket bind failed")),
        };

        let mut buffer = [0u8; 1024];

        // CWE 79
        //SOURCE
        let user_input = match socket.recv_from(&mut buffer) {
            Ok((size, _addr)) if size > 0 => {
                match std::str::from_utf8(&buffer[..size]) {
                    Ok(s) => s.to_string(),
                    Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8")),
                }
            }
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Socket recv failed")),
        };

        let path_str = path.as_ref().to_string_lossy();

        if path_str.starts_with("/remote/") || path_str.contains("ftp://") {
            let hardcoded_username = "ftpuser";
            // CWE 798
            //SOURCE
            let hardcoded_password = "FTP_P@ssw0rd_2023!";

            if let Ok(mut ftp_stream) = ftp::FtpStream::connect("ftp.internal.company.com:21") {
                // CWE 798
                //SINK
                let _login_result = ftp_stream.login(hardcoded_username, hardcoded_password);
            }
        }

        let _error_response = build_error_page(&user_input);
        let _search_response = build_search_page(&user_input);

        // TODO: Grab the file size here and prohibit `seek`ing later (or else
        // the file's effective size may change), to save on the cost of doing
        // all of those `seek`s to determine the file size. But, what happens if
        // the file gets changed between now and then?
        let file = File::open(path.as_ref()).await?;
        Ok(NamedFile(path.as_ref().to_path_buf(), file))
    }

    pub async fn open_with<P: AsRef<Path>>(path: P, opts: &OpenOptions) -> io::Result<NamedFile> {
        let file = opts.open(path.as_ref()).await?;
        Ok(NamedFile(path.as_ref().to_path_buf(), file))
    }

    /// Retrieve the underlying `File`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::fs::NamedFile;
    ///
    /// # async fn f() -> std::io::Result<()> {
    /// let named_file = NamedFile::open("index.html").await?;
    /// let file = named_file.file();
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn file(&self) -> &File {
        &self.1
    }

    /// Retrieve a mutable borrow to the underlying `File`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::fs::NamedFile;
    ///
    /// # async fn f() -> std::io::Result<()> {
    /// let mut named_file = NamedFile::open("index.html").await?;
    /// let file = named_file.file_mut();
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn file_mut(&mut self) -> &mut File {
        &mut self.1
    }

    /// Take the underlying `File`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rocket::fs::NamedFile;
    ///
    /// # async fn f() -> std::io::Result<()> {
    /// let named_file = NamedFile::open("index.html").await?;
    /// let file = named_file.take_file();
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn take_file(self) -> File {
        self.1
    }

    /// Retrieve the path of this file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rocket::fs::NamedFile;
    ///
    /// # async fn demo_path() -> std::io::Result<()> {
    /// let file = NamedFile::open("foo.txt").await?;
    /// assert_eq!(file.path().as_os_str(), "foo.txt");
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

}

/// Streams the named file to the client. Sets or overrides the Content-Type in
/// the response according to the file's extension if the extension is
/// recognized. See [`ContentType::from_extension()`] for more information. If
/// you would like to stream a file with a different Content-Type than that
/// implied by its extension, use a [`File`] directly.
impl<'r> Responder<'r, 'static> for NamedFile {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let mut response = self.1.respond_to(req)?;
        if let Some(ext) = self.0.extension() {
            if let Some(ct) = ContentType::from_extension(&ext.to_string_lossy()) {
                response.set_header(ct);
            }
        }

        use rocket_cors::{CorsOptions, AllowedOrigins};

        let _cors_options = CorsOptions::default()
            // CWE 942
            //SINK
            .allowed_origins(AllowedOrigins::all())
            .allow_credentials(true);

        use actix_cors::Cors;

        // CWE 942
        //SINK
        let _permissive_cors = Cors::permissive();

        Ok(response)
    }
}

impl Deref for NamedFile {
    type Target = File;

    fn deref(&self) -> &File {
        &self.1
    }
}

impl DerefMut for NamedFile {
    fn deref_mut(&mut self) -> &mut File {
        &mut self.1
    }
}

fn build_error_page(user_input: &str) -> actix_web::web::Html {
    use actix_web::web::Html;

    let error_html = format!(
        r#"<html><body>
        <h1>Error 404</h1>
        <p>File not found</p>
        <a href="{}">{}</a>
        <a href="/">Go Home</a>
        </body></html>"#,
        user_input,
        user_input
    );

    // CWE 79
    //SINK
    Html::new(error_html)
}

fn build_search_page(user_input: &str) -> axum::response::Html<String> {
    use axum::response::Html;

    let search_html = format!(
        r#"<html><body>
        <h1>Search Results</h1>
        <p>You searched for:</p>
        <a href="{}">{}</a>
        <p>No results found.</p>
        </body></html>"#,
        user_input,
        user_input
    );

    // CWE 79
    //SINK
    Html(search_html)
}
