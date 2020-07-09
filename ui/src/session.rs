use seed::prelude::StreamHandle;

////////////////////////////////////////////////////////////////
// TYPES //
////////////////////////////////////////////////////////////////

pub struct Session {
    api_url: &'static str,

    /// Not the time stamp since 1970
    /// rather, the time stamp since the
    /// beginning of browser session
    timestamp: f64,
    timestamp_delta: f64,
    window_size: WindowSize,
    errors: Vec<Error>,
}

enum Error {
    Error(String),
}

pub struct WindowSize {
    pub width: u16,
    pub height: u16,
}

////////////////////////////////////////////////////////////////
// CONSTS //
////////////////////////////////////////////////////////////////

static DEV_API_URL: &str = "http://localhost:2943";
static FPS_24: f64 = 41.6667;

////////////////////////////////////////////////////////////////
// INIT //
////////////////////////////////////////////////////////////////

pub fn init(window_size: WindowSize) -> Session {
    Session {
        api_url: DEV_API_URL,
        timestamp: 0.0,
        timestamp_delta: 0.0,
        window_size,
        errors: Vec::new(),
    }
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

impl Session {
    pub fn record_error(&mut self, error: String) -> &mut Session {
        self.errors.push(Error::Error(error));
        self
    }

    pub fn url(&self, path: &str) -> String {
        let mut buf: String = String::new();

        buf.push_str(self.api_url);
        buf.push_str(path);

        buf
    }

    pub fn set_window_size(&mut self, window_size: WindowSize) -> &mut Session {
        self.window_size = window_size;
        self
    }

    pub fn get_window_size(&self) -> &WindowSize {
        &self.window_size
    }

    pub fn set_current_time(&mut self, timestamp: f64) -> &mut Session {
        self.timestamp = timestamp;
        self
    }

    pub fn set_render_delta(&mut self, maybe_delta: Option<f64>) -> &mut Session {
        if let Some(delta) = maybe_delta {
            self.timestamp_delta = delta;
        }
        self
    }

    pub fn get_current_time(&self) -> f64 {
        self.timestamp
    }

    pub fn get_fps_str(&self) -> String {
        let mut buf = String::new();

        buf.push_str((1000.0 / self.timestamp_delta).round().to_string().as_str());
        buf.push_str("FPS");
        buf
    }
    pub fn get_frame(&self) -> i64 {
        (self.get_current_time() / FPS_24) as i64
    }

    pub fn asset_url(&self, file_name: &'static str) -> String {
        let mut path = String::new();
        path.push_str("/assets/");
        path.push_str(file_name);
        path.push_str(".png");

        self.url(path.as_str())
    }
}

////////////////////////////////////////////////////////////////
// Tests //
////////////////////////////////////////////////////////////////

#[cfg(test)]
mod session_tests {
    use crate::session::{init, Session, WindowSize, FPS_24};

    fn init_test() -> Session {
        init(WindowSize {
            width: 1440,
            height: 800,
        })
    }

    #[test]
    fn within_first_frame() {
        let mut session = init_test();

        session.set_current_time(FPS_24 - 0.001);

        assert_eq!(session.get_frame(), 0);
    }
    #[test]
    fn after_first_frame() {
        let mut session = init_test();

        session.set_current_time(FPS_24 + 0.001);

        assert_eq!(session.get_frame(), 1);
    }
}
