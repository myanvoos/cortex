
// This enum represents the different sections of the document
// Right now, there's the setup section that allows you to set up the document and specify variables and functions
// Then document is for rendering

enum Section {
    Setup, 
    Document,
}

struct Document {
    title: String,
    author: String,
    expressions: Vec<String>,
    children: Vec<Document> // allow nested documents
}

// Init a new document

impl Document {
    fn new() -> Self {
        Document {
            title: String::new(),
            author: String::new(),
            expressions: Vec::new(),
            children: Vec::new(),
        }
    }
}