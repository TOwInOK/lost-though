
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 74,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.74.0 (79e9716c9 2023-11-13)".to_owned(),
                    commit_hash: Some("79e9716c980570bfd1f666e3b16ac583f0168962".to_owned()),
                    commit_date: Some("2023-11-13".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            