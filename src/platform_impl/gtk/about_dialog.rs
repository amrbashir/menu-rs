use gtk::prelude::*;

use crate::AboutMetadata;

/// Displays an about dialog using GTK with the provided metadata.
pub struct AboutDialog {
    metadata: AboutMetadata,
}

impl AboutDialog {
    /// Create a new about dialog using `metadata` but without showing it.
    pub fn new(metadata: AboutMetadata) -> AboutDialog {
        AboutDialog { metadata }
    }

    /// Show the about dialog and block until it is closed.
    pub fn show(&self) {
        let mut builder = gtk::AboutDialog::builder().modal(true).resizable(false);

        if let Some(name) = &self.metadata.name {
            builder = builder.program_name(name);
        }
        if let Some(version) = &self.metadata.full_version() {
            builder = builder.version(version);
        }
        if let Some(authors) = &self.metadata.authors {
            builder = builder.authors(authors.clone());
        }
        if let Some(comments) = &self.metadata.comments {
            builder = builder.comments(comments);
        }
        if let Some(copyright) = &self.metadata.copyright {
            builder = builder.copyright(copyright);
        }
        if let Some(license) = &self.metadata.license {
            builder = builder.license(license);
        }
        if let Some(website) = &self.metadata.website {
            builder = builder.website(website);
        }
        if let Some(website_label) = &self.metadata.website_label {
            builder = builder.website_label(website_label);
        }
        if let Some(icon) = &self.metadata.icon {
            builder = builder.logo(&icon.to_pixbuf());
        }

        let about = builder.build();

        about.run();

        unsafe {
            about.destroy();
        }
    }
}
