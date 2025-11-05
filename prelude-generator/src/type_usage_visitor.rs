use std::collections::{HashMap, HashSet, BTreeSet};
use syn::{self, visit::{self, Visit}, ItemFn, ItemStruct, ItemEnum, ItemConst, ItemStatic, Type, Expr, ItemImpl};
use quote::ToTokens;
use crate::expression_info::ExpressionInfo;
use crate::type_collector::TypeCollector;
use crate::{struct_lattice_info::StructLatticeInfo, enum_lattice_info::EnumLatticeInfo, impl_lattice_info::ImplLatticeInfo};

// Helper visitor for struct field co-occurrence
struct StructFieldCoOccurrenceVisitor<'a> {
    _struct_name: &'a str,
    current_field_accesses: BTreeSet<String>, // Use BTreeSet for hashable key
    struct_lattice_info: &'a mut StructLatticeInfo,
}

impl<'a, 'ast> Visit<'ast> for StructFieldCoOccurrenceVisitor<'a> {
    fn visit_expr_field(&mut self, i: &'ast syn::ExprField) {
        // This is a very basic check. A more robust solution would involve type resolution
        // to ensure 'base_ident' actually refers to an instance of 'self.struct_name'.
        // For now, we'll assume any field access within the context of this visitor
        // is relevant to the struct being analyzed.
        if let Some(field_ident) = i.member.clone().into_token_stream().to_string().strip_prefix(".").map(|s| s.to_string()) {
            self.current_field_accesses.insert(field_ident);
        }
        visit::visit_expr_field(self, i);
    }

    // When a method or block finishes, we can record the co-occurrence.
    // For simplicity in this initial pass, we'll just collect all field accesses within a method.
    // A more refined approach would analyze co-occurrence within smaller expression units.
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // Before visiting the function body, clear previous accesses
        self.current_field_accesses.clear();
        visit::visit_item_fn(self, i);
        // After visiting the function body, record co-occurrences if any fields were accessed
        if !self.current_field_accesses.is_empty() {
            self.struct_lattice_info.add_co_occurrence(self.current_field_accesses.clone());
        }
    }
}

// Helper visitor for enum variant co-occurrence
struct EnumVariantCoOccurrenceVisitor<'a> {
    _enum_name: &'a str,
    _current_variant_types: BTreeSet<String>, // Use BTreeSet for hashable key
    enum_lattice_info: &'a mut EnumLatticeInfo,
}

impl<'a, 'ast> Visit<'ast> for EnumVariantCoOccurrenceVisitor<'a> {
    fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
        // For each match arm, collect the types of the variants being matched
        let mut matched_variant_types = BTreeSet::new();
        for arm in &i.arms {
            if let syn::Pat::TupleStruct(pat_tuple_struct) = &arm.pat {
                if let Some(segment) = pat_tuple_struct.path.segments.last() {
                    matched_variant_types.insert(segment.ident.to_string());
                }
            } else if let syn::Pat::Path(pat_path) = &arm.pat {
                if let Some(segment) = pat_path.path.segments.last() {
                    matched_variant_types.insert(segment.ident.to_string());
                }
            }
        }
        if !matched_variant_types.is_empty() {
            self.enum_lattice_info.add_co_occurrence(matched_variant_types);
        }
        visit::visit_expr_match(self, i);
    }

    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        // Check for if let patterns
        if let syn::Expr::Let(expr_let) = &*i.cond {
            if let syn::Pat::TupleStruct(pat_tuple_struct) = &*expr_let.pat {
                if let Some(segment) = pat_tuple_struct.path.segments.last() {
                    self.enum_lattice_info.add_co_occurrence(BTreeSet::from([segment.ident.to_string()]));
                }
            } else if let syn::Pat::Path(pat_path) = &*expr_let.pat {
                if let Some(segment) = pat_path.path.segments.last() {
                    self.enum_lattice_info.add_co_occurrence(BTreeSet::from([segment.ident.to_string()]));
                }
            }
        }
        visit::visit_expr_if(self, i);
    }
}

// Helper visitor for impl method co-occurrence
struct ImplMethodCoOccurrenceVisitor<'a> {
    _impl_for_type: &'a str,
    current_method_calls: BTreeSet<String>, // Use BTreeSet for hashable key
    impl_lattice_info: &'a mut ImplLatticeInfo,
}

impl<'a, 'ast> Visit<'ast> for ImplMethodCoOccurrenceVisitor<'a> {
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        self.current_method_calls.insert(i.method.to_string());
        visit::visit_expr_method_call(self, i);
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        // Before visiting the function body, clear previous calls
        self.current_method_calls.clear();
        visit::visit_item_fn(self, i);
        // After visiting the function body, record co-occurrences if any methods were called
        if !self.current_method_calls.is_empty() {
            self.impl_lattice_info.add_co_occurrence(self.current_method_calls.clone());
        }
    }
}

pub struct TypeUsageVisitor {
    pub max_depth: usize,
    pub current_depth: usize,
    pub expressions: HashMap<String, ExpressionInfo>,
    pub struct_lattices: HashMap<String, StructLatticeInfo>,
    pub enum_lattices: HashMap<String, EnumLatticeInfo>,
    pub impl_lattices: HashMap<String, ImplLatticeInfo>,
}

impl TypeUsageVisitor {
    pub fn new(max_depth: usize) -> Self {
        TypeUsageVisitor {
            max_depth,
            current_depth: 0,
            expressions: HashMap::new(),
            struct_lattices: HashMap::new(),
            enum_lattices: HashMap::new(),
            impl_lattices: HashMap::new(),
        }
    }

    pub fn get_expr_node_type(expr: &Expr) -> String {
        match expr {
            Expr::Array(_) => "Array".to_string(),
            Expr::Assign(_) => "Assign".to_string(),
            Expr::Async(_) => "Async".to_string(),
            Expr::Await(_) => "Await".to_string(),
            Expr::Binary(_) => "Binary".to_string(),
            Expr::Block(_) => "Block".to_string(),
            Expr::Break(_) => "Break".to_string(),
            Expr::Call(_) => "Call".to_string(),
            Expr::Cast(_) => "Cast".to_string(),
            Expr::Closure(_) => "Closure".to_string(),
            Expr::Continue(_) => "Continue".to_string(),
            Expr::Field(_) => "Field".to_string(),
            Expr::ForLoop(_) => "ForLoop".to_string(),
            Expr::Group(_) => "Group".to_string(),
            Expr::If(_) => "If".to_string(),
            Expr::Index(_) => "Index".to_string(),
            Expr::Infer(_) => "Infer".to_string(),
            Expr::Let(_) => "Let".to_string(),
            Expr::Lit(_) => "Lit".to_string(),
            Expr::Loop(_) => "Loop".to_string(),
            Expr::Macro(_) => "Macro".to_string(),
            Expr::Match(_) => "Match".to_string(),
            Expr::MethodCall(_) => "MethodCall".to_string(),
            Expr::Paren(_) => "Paren".to_string(),
            Expr::Path(_) => "Path".to_string(),
            Expr::Range(_) => "Range".to_string(),
            Expr::Reference(_) => "Reference".to_string(),
            Expr::Repeat(_) => "Repeat".to_string(),
            Expr::Return(_) => "Return".to_string(),
            Expr::Struct(_) => "Struct".to_string(),
            Expr::Tuple(_) => "Tuple".to_string(),
            Expr::Unary(_) => "Unary".to_string(),
            Expr::Unsafe(_) => "Unsafe".to_string(),
            Expr::Verbatim(_) => "Verbatim".to_string(),
            Expr::While(_) => "While".to_string(),
            Expr::Yield(_) => "Yield".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub fn extract_types_from_expr(&self, expr: &Expr) -> HashSet<String> {
        let mut types = HashSet::new();
        let mut collector = TypeCollector { types: &mut types };
        collector.visit_expr(expr);
        types
    }

    pub fn process_expression(&mut self, expr: &Expr) {
        if self.current_depth <= self.max_depth {
            let expr_str = expr.to_token_stream().to_string();
            let used_types = self.extract_types_from_expr(expr);
            let other_types_count = used_types.len();
            let node_type = Self::get_expr_node_type(expr);

            let info = ExpressionInfo {
                expression_str: expr_str.clone(),
                depth: self.current_depth,
                used_types,
                other_types_count,
                node_type,
            };
            self.expressions.insert(expr_str, info);
        }
    }
}

impl<'ast> Visit<'ast> for TypeUsageVisitor {
    fn visit_expr(&mut self, i: &'ast Expr) {
        self.current_depth += 1;
        self.process_expression(i);
        visit::visit_expr(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        self.current_depth += 1;
        visit::visit_item_fn(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        self.current_depth += 1;
        let struct_name = i.ident.to_string();
        self.struct_lattices.entry(struct_name.clone()).or_insert_with(|| StructLatticeInfo::new(struct_name));
        visit::visit_item_struct(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_enum(&mut self, i: &'ast ItemEnum) {
        self.current_depth += 1;
        let enum_name = i.ident.to_string();
        let enum_lattice_info = self.enum_lattices.entry(enum_name.clone()).or_insert_with(|| EnumLatticeInfo::new(enum_name.clone()));

        let mut sub_visitor = EnumVariantCoOccurrenceVisitor {
            _enum_name: &enum_name,
            _current_variant_types: BTreeSet::new(),
            enum_lattice_info,
        };
        sub_visitor.visit_item_enum(i);

        visit::visit_item_enum(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_const(&mut self, i: &'ast ItemConst) {
        self.current_depth += 1;
        visit::visit_item_const(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_static(&mut self, i: &'ast ItemStatic) {
        self.current_depth += 1;
        visit::visit_item_static(self, i);
        self.current_depth -= 1;
    }

    fn visit_item_impl(&mut self, i: &'ast ItemImpl) {
        self.current_depth += 1;
        let impl_for_type = if let Type::Path(type_path) = &*i.self_ty {
            type_path.path.segments.last().map(|segment| segment.ident.to_string())
        } else {
            None
        };

        if let Some(impl_for_type_name) = impl_for_type {
            // Update impl_lattices
            let impl_lattice_info = self.impl_lattices.entry(impl_for_type_name.clone()).or_insert_with(|| ImplLatticeInfo::new(impl_for_type_name.clone()));

            // Run the ImplMethodCoOccurrenceVisitor
            let mut sub_visitor = ImplMethodCoOccurrenceVisitor {
                _impl_for_type: &impl_for_type_name,
                current_method_calls: BTreeSet::new(),
                impl_lattice_info,
            };
            sub_visitor.visit_item_impl(i);

            // If this impl is for a struct, run the StructFieldCoOccurrenceVisitor
            if let Some(struct_lattice_info) = self.struct_lattices.get_mut(&impl_for_type_name) {
                let mut sub_visitor = StructFieldCoOccurrenceVisitor {
                    _struct_name: &impl_for_type_name,
                    current_field_accesses: BTreeSet::new(),
                    struct_lattice_info,
                };
                sub_visitor.visit_item_impl(i);
            }
        }
        visit::visit_item_impl(self, i);
        self.current_depth -= 1;
    }
}
