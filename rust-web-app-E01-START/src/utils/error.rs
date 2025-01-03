    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug)]
    pub enum Error {
        DateFailParse(String),
        FaildedToB64uDecode,
    }

    impl core::fmt::Display for Error {
        fn fmt(
            &self, 
            f: &mut core::fmt::Formatter<'_>
        ) -> core::result::Result<(), core::fmt::Error> {
            write!(f, "{self:?}")
        }
    }

    impl std::error::Error for Error {}