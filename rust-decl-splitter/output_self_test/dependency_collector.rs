
impl DependencyCollector { pub fn new () -> Self { DependencyCollector { dependencies : HashSet :: new () , } } fn add_dependency (& mut self , ident : & Ident) { self . dependencies . insert (ident . to_string ()) ; } }

impl < 'ast > Visit < 'ast > for DependencyCollector { fn visit_path (& mut self , i : & 'ast Path) { for segment in & i . segments { self . add_dependency (& segment . ident) ; } visit :: visit_path (self , i) ; } fn visit_macro (& mut self , i : & 'ast Macro) { for segment in & i . path . segments { self . add_dependency (& segment . ident) ; } visit :: visit_macro (self , i) ; } fn visit_ident (& mut self , i : & 'ast Ident) { self . add_dependency (i) ; } fn visit_item (& mut self , i : & 'ast Item) { visit :: visit_item (self , i) ; } }
