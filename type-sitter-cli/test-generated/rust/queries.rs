#[allow(non_upper_case_globals)]
static __Tags__: type_sitter_lib::gen_internal::TypedQueryOnceBox<tree_sitter::Query> =
    type_sitter_lib::gen_internal::TypedQueryOnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Tags() -> Box<tree_sitter::Query> {
    # [allow (unused_mut)] let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    Box::new(query)
}
#[doc = "Typed version of the query:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Tags;
#[doc = "Matches returned by a query cursor running the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type TagsMatches<'cursor, 'tree> = type_sitter_lib::TypedQueryMatches<'cursor, 'tree, Tags>;
#[doc = "Captures returned by a query cursor running the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type TagsCaptures<'cursor, 'tree> = type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, Tags>;
#[doc = "A match returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
pub struct TagsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
#[doc = "A capture returned by the query [Tags]:\n\n```sexp\n; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n\n```"]
pub enum TagsCapture<'cursor, 'tree> {
    #[doc = "A `name` ([anon_unions::Name])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    Name {
        node: anon_unions::Name<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.class` ([anon_unions::DefinitionClass])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    DefinitionClass {
        node: anon_unions::DefinitionClass<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.method` ([super::nodes::DeclarationList])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method"]
    #[doc = "```"]
    DefinitionMethod {
        node: super::nodes::DeclarationList<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.function` ([super::nodes::FunctionItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    DefinitionFunction {
        node: super::nodes::FunctionItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.interface` ([super::nodes::TraitItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    DefinitionInterface {
        node: super::nodes::TraitItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.module` ([super::nodes::ModItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    DefinitionModule {
        node: super::nodes::ModItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `definition.macro` ([super::nodes::MacroDefinition])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    DefinitionMacro {
        node: super::nodes::MacroDefinition<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `reference.call` ([anon_unions::ReferenceCall])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    ReferenceCall {
        node: anon_unions::ReferenceCall<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `reference.implementation` ([anon_unions::ReferenceImplementation])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    ReferenceImplementation {
        node: super::nodes::ImplItem<'tree>,
        match_: Option<TagsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Tags {
    type Match<'cursor, 'tree: 'cursor> = TagsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree: 'cursor> = TagsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "; ADT definitions\n\n(struct_item\n    name: (type_identifier) @name) @definition.class\n\n(enum_item\n    name: (type_identifier) @name) @definition.class\n\n(union_item\n    name: (type_identifier) @name) @definition.class\n\n; type aliases\n\n(type_item\n    name: (type_identifier) @name) @definition.class\n\n; method definitions\n\n(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method\n\n; function definitions\n\n(function_item\n    name: (identifier) @name) @definition.function\n\n; trait definitions\n(trait_item\n    name: (type_identifier) @name) @definition.interface\n\n; module definitions\n(mod_item\n    name: (identifier) @name) @definition.module\n\n; macro definitions\n\n(macro_definition\n    name: (identifier) @name) @definition.macro\n\n; references\n\n(call_expression\n    function: (identifier) @name) @reference.call\n\n(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call\n\n(macro_invocation\n    macro: (identifier) @name) @reference.call\n\n; implementations\n\n(impl_item\n    trait: (type_identifier) @name) @reference.implementation\n\n(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Tags__.get_or_init(__Mk__Tags)
    }
    #[inline]
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        match_: tree_sitter::QueryMatch<'cursor, 'tree>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_, tree }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index as usize { 0usize => Self :: Capture :: Name { node : < anon_unions :: Name < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 1usize => Self :: Capture :: DefinitionClass { node : < anon_unions :: DefinitionClass < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 2usize => Self :: Capture :: DefinitionMethod { node : < super :: nodes :: DeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 3usize => Self :: Capture :: DefinitionFunction { node : < super :: nodes :: FunctionItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 4usize => Self :: Capture :: DefinitionInterface { node : < super :: nodes :: TraitItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 5usize => Self :: Capture :: DefinitionModule { node : < super :: nodes :: ModItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 6usize => Self :: Capture :: DefinitionMacro { node : < super :: nodes :: MacroDefinition < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 7usize => Self :: Capture :: ReferenceCall { node : < anon_unions :: ReferenceCall < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 8usize => Self :: Capture :: ReferenceImplementation { node : < super :: nodes :: ImplItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TagsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `name` ([anon_unions::Name])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn name(&self) -> Option<anon_unions::Name<'tree>> {
        { [0u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Name < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.class` ([anon_unions::DefinitionClass])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_class(&self) -> Option<anon_unions::DefinitionClass<'tree>> {
        { [1u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: DefinitionClass < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.method` ([super::nodes::DeclarationList])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_method(&self) -> Option<super::nodes::DeclarationList<'tree>> {
        { [2u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: DeclarationList < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.function` ([super::nodes::FunctionItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_function(&self) -> Option<super::nodes::FunctionItem<'tree>> {
        { [3u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FunctionItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.interface` ([super::nodes::TraitItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_interface(&self) -> Option<super::nodes::TraitItem<'tree>> {
        { [4u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TraitItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.module` ([super::nodes::ModItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_module(&self) -> Option<super::nodes::ModItem<'tree>> {
        { [5u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: ModItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `definition.macro` ([super::nodes::MacroDefinition])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_macro(&self) -> Option<super::nodes::MacroDefinition<'tree>> {
        { [6u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: MacroDefinition < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `reference.call` ([anon_unions::ReferenceCall])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_call(&self) -> Option<anon_unions::ReferenceCall<'tree>> {
        { [7u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: ReferenceCall < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `reference.implementation` ([anon_unions::ReferenceImplementation])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_implementation(&self) -> Option<super::nodes::ImplItem<'tree>> {
        { [8u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: ImplItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for TagsMatch<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(TagsMatch))
            .field("match_", &self.match_)
            .finish()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
    for TagsMatch<'cursor, 'tree>
{
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Tags
    }
    #[inline]
    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
        self.tree
    }
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.match_
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.match_
    }
}
#[automatically_derived]
impl<'cursor, 'tree> TagsCapture<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `name` ([anon_unions::Name])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(field_identifier) @name"]
    #[doc = "(identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "(type_identifier) @name"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn name(&self) -> Option<&anon_unions::Name<'tree>> {
        match self {
            Self::Name { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.class` ([anon_unions::DefinitionClass])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(struct_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(enum_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(union_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "(type_item\n    name: (type_identifier) @name) @definition.class"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_class(&self) -> Option<&anon_unions::DefinitionClass<'tree>> {
        match self {
            Self::DefinitionClass { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.method` ([super::nodes::DeclarationList])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(declaration_list\n    (function_item\n        name: (identifier) @name)) @definition.method"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_method(&self) -> Option<&super::nodes::DeclarationList<'tree>> {
        match self {
            Self::DefinitionMethod { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.function` ([super::nodes::FunctionItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(function_item\n    name: (identifier) @name) @definition.function"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_function(&self) -> Option<&super::nodes::FunctionItem<'tree>> {
        match self {
            Self::DefinitionFunction { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.interface` ([super::nodes::TraitItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(trait_item\n    name: (type_identifier) @name) @definition.interface"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_interface(&self) -> Option<&super::nodes::TraitItem<'tree>> {
        match self {
            Self::DefinitionInterface { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.module` ([super::nodes::ModItem])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(mod_item\n    name: (identifier) @name) @definition.module"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_module(&self) -> Option<&super::nodes::ModItem<'tree>> {
        match self {
            Self::DefinitionModule { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `definition.macro` ([super::nodes::MacroDefinition])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(macro_definition\n    name: (identifier) @name) @definition.macro"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn definition_macro(&self) -> Option<&super::nodes::MacroDefinition<'tree>> {
        match self {
            Self::DefinitionMacro { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `reference.call` ([anon_unions::ReferenceCall])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(call_expression\n    function: (identifier) @name) @reference.call"]
    #[doc = "(call_expression\n    function: (field_expression\n        field: (field_identifier) @name)) @reference.call"]
    #[doc = "(macro_invocation\n    macro: (identifier) @name) @reference.call"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_call(&self) -> Option<&anon_unions::ReferenceCall<'tree>> {
        match self {
            Self::ReferenceCall { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `reference.implementation` ([anon_unions::ReferenceImplementation])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(impl_item\n    trait: (type_identifier) @name) @reference.implementation"]
    #[doc = "(impl_item\n    type: (type_identifier) @name\n    !trait) @reference.implementation"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_implementation(&self) -> Option<&super::nodes::ImplItem<'tree>> {
        match self {
            Self::ReferenceImplementation { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for TagsCapture<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name { node, .. } => f
                .debug_struct(concat!(stringify!(TagsCapture), "::", stringify!(Name)))
                .field("node", node)
                .finish(),
            Self::DefinitionClass { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionClass)
                ))
                .field("node", node)
                .finish(),
            Self::DefinitionMethod { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionMethod)
                ))
                .field("node", node)
                .finish(),
            Self::DefinitionFunction { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionFunction)
                ))
                .field("node", node)
                .finish(),
            Self::DefinitionInterface { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionInterface)
                ))
                .field("node", node)
                .finish(),
            Self::DefinitionModule { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionModule)
                ))
                .field("node", node)
                .finish(),
            Self::DefinitionMacro { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(DefinitionMacro)
                ))
                .field("node", node)
                .finish(),
            Self::ReferenceCall { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(ReferenceCall)
                ))
                .field("node", node)
                .finish(),
            Self::ReferenceImplementation { node, .. } => f
                .debug_struct(concat!(
                    stringify!(TagsCapture),
                    "::",
                    stringify!(ReferenceImplementation)
                ))
                .field("node", node)
                .finish(),
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for TagsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Name { node, .. } => Self::Name {
                node: *node,
                match_: None,
            },
            Self::DefinitionClass { node, .. } => Self::DefinitionClass {
                node: *node,
                match_: None,
            },
            Self::DefinitionMethod { node, .. } => Self::DefinitionMethod {
                node: *node,
                match_: None,
            },
            Self::DefinitionFunction { node, .. } => Self::DefinitionFunction {
                node: *node,
                match_: None,
            },
            Self::DefinitionInterface { node, .. } => Self::DefinitionInterface {
                node: *node,
                match_: None,
            },
            Self::DefinitionModule { node, .. } => Self::DefinitionModule {
                node: *node,
                match_: None,
            },
            Self::DefinitionMacro { node, .. } => Self::DefinitionMacro {
                node: *node,
                match_: None,
            },
            Self::ReferenceCall { node, .. } => Self::ReferenceCall {
                node: *node,
                match_: None,
            },
            Self::ReferenceImplementation { node, .. } => Self::ReferenceImplementation {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for TagsCapture<'cursor, 'tree>
{
    type Query = Tags;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Tags
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Name { match_, .. } => match_.as_ref(),
            Self::DefinitionClass { match_, .. } => match_.as_ref(),
            Self::DefinitionMethod { match_, .. } => match_.as_ref(),
            Self::DefinitionFunction { match_, .. } => match_.as_ref(),
            Self::DefinitionInterface { match_, .. } => match_.as_ref(),
            Self::DefinitionModule { match_, .. } => match_.as_ref(),
            Self::DefinitionMacro { match_, .. } => match_.as_ref(),
            Self::ReferenceCall { match_, .. } => match_.as_ref(),
            Self::ReferenceImplementation { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Name { match_, .. } => match_,
            Self::DefinitionClass { match_, .. } => match_,
            Self::DefinitionMethod { match_, .. } => match_,
            Self::DefinitionFunction { match_, .. } => match_,
            Self::DefinitionInterface { match_, .. } => match_,
            Self::DefinitionModule { match_, .. } => match_,
            Self::DefinitionMacro { match_, .. } => match_,
            Self::ReferenceCall { match_, .. } => match_,
            Self::ReferenceImplementation { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(&self) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 0usize,
                name: "name",
            },
            Self::DefinitionClass { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 1usize,
                    name: "definition.class",
                }
            }
            Self::DefinitionMethod { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 2usize,
                    name: "definition.method",
                }
            }
            Self::DefinitionFunction { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 3usize,
                    name: "definition.function",
                }
            }
            Self::DefinitionInterface { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 4usize,
                    name: "definition.interface",
                }
            }
            Self::DefinitionModule { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 5usize,
                    name: "definition.module",
                }
            }
            Self::DefinitionMacro { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 6usize,
                    name: "definition.macro",
                }
            }
            Self::ReferenceCall { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 7usize,
                    name: "reference.call",
                }
            }
            Self::ReferenceImplementation { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 8usize,
                    name: "reference.implementation",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => node.node(),
            Self::DefinitionClass { node, .. } => node.node(),
            Self::DefinitionMethod { node, .. } => node.node(),
            Self::DefinitionFunction { node, .. } => node.node(),
            Self::DefinitionInterface { node, .. } => node.node(),
            Self::DefinitionModule { node, .. } => node.node(),
            Self::DefinitionMacro { node, .. } => node.node(),
            Self::ReferenceCall { node, .. } => node.node(),
            Self::ReferenceImplementation { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Name { node, .. } => node.node_mut(),
            Self::DefinitionClass { node, .. } => node.node_mut(),
            Self::DefinitionMethod { node, .. } => node.node_mut(),
            Self::DefinitionFunction { node, .. } => node.node_mut(),
            Self::DefinitionInterface { node, .. } => node.node_mut(),
            Self::DefinitionModule { node, .. } => node.node_mut(),
            Self::DefinitionMacro { node, .. } => node.node_mut(),
            Self::ReferenceCall { node, .. } => node.node_mut(),
            Self::ReferenceImplementation { node, .. } => node.node_mut(),
        }
    }
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Name { .. } => "name",
            Self::DefinitionClass { .. } => "definition.class",
            Self::DefinitionMethod { .. } => "definition.method",
            Self::DefinitionFunction { .. } => "definition.function",
            Self::DefinitionInterface { .. } => "definition.interface",
            Self::DefinitionModule { .. } => "definition.module",
            Self::DefinitionMacro { .. } => "definition.macro",
            Self::ReferenceCall { .. } => "reference.call",
            Self::ReferenceImplementation { .. } => "reference.implementation",
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Name { .. } => 0usize,
            Self::DefinitionClass { .. } => 1usize,
            Self::DefinitionMethod { .. } => 2usize,
            Self::DefinitionFunction { .. } => 3usize,
            Self::DefinitionInterface { .. } => 4usize,
            Self::DefinitionModule { .. } => 5usize,
            Self::DefinitionMacro { .. } => 6usize,
            Self::ReferenceCall { .. } => 7usize,
            Self::ReferenceImplementation { .. } => 8usize,
        }
    }
}
#[allow(non_upper_case_globals)]
static __Highlights__: type_sitter_lib::gen_internal::TypedQueryOnceBox<tree_sitter::Query> =
    type_sitter_lib::gen_internal::TypedQueryOnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Highlights() -> Box<tree_sitter::Query> {
    # [allow (unused_mut)] let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    Box::new(query)
}
#[doc = "Typed version of the query:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Highlights;
#[doc = "Matches returned by a query cursor running the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsMatches<'cursor, 'tree> =
    type_sitter_lib::TypedQueryMatches<'cursor, 'tree, Highlights>;
#[doc = "Captures returned by a query cursor running the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type HighlightsCaptures<'cursor, 'tree> =
    type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, Highlights>;
#[doc = "A match returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
pub struct HighlightsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
#[doc = "A capture returned by the query [Highlights]:\n\n```sexp\n; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n\n```"]
pub enum HighlightsCapture<'cursor, 'tree> {
    #[doc = "A `constant` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    Constant {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `type` ([anon_unions::Type])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    Type {
        node: anon_unions::Type<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `constructor` ([anon_unions::Constructor])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    Constructor {
        node: anon_unions::Constructor<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function` ([anon_unions::Function])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    Function {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function.method` ([anon_unions::FunctionMethod])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    FunctionMethod {
        node: super::nodes::FieldIdentifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `function.macro` ([anon_unions::FunctionMacro])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    FunctionMacro {
        node: anon_unions::FunctionMacro<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `type.builtin` ([super::nodes::PrimitiveType])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    TypeBuiltin {
        node: super::nodes::PrimitiveType<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `property` ([super::nodes::FieldIdentifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    Property {
        node: super::nodes::FieldIdentifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `comment` ([anon_unions::Comment])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    Comment {
        node: anon_unions::Comment<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `punctuation.bracket` ([anon_unions::PunctuationBracket])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    PunctuationBracket {
        node: anon_unions::PunctuationBracket<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `punctuation.delimiter` ([anon_unions::PunctuationDelimiter])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    PunctuationDelimiter {
        node: anon_unions::PunctuationDelimiter<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `variable.parameter` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    VariableParameter {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `label` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    Label {
        node: super::nodes::Identifier<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `keyword` ([anon_unions::Keyword])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "```"]
    Keyword {
        node: anon_unions::Keyword<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `variable.builtin` ([super::nodes::_Self])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    VariableBuiltin {
        node: super::nodes::_Self<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `string` ([anon_unions::String])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    String {
        node: anon_unions::String<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `constant.builtin` ([anon_unions::ConstantBuiltin])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
    #[doc = "```"]
    ConstantBuiltin {
        node: anon_unions::ConstantBuiltin<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `escape` ([super::nodes::EscapeSequence])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    Escape {
        node: super::nodes::EscapeSequence<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `attribute` ([anon_unions::Attribute])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    Attribute {
        node: anon_unions::Attribute<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
    #[doc = "A `operator` ([anon_unions::Operator])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    Operator {
        node: anon_unions::Operator<'tree>,
        match_: Option<HighlightsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Highlights {
    type Match<'cursor, 'tree: 'cursor> = HighlightsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree: 'cursor> = HighlightsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "; Identifier conventions\n\n; Assume all-caps names are constants\n((identifier) @constant\n (#match? @constant \"^[A-Z][A-Z\\\\d_]+$'\"))\n\n; Assume that uppercase names in paths are types\n((scoped_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (identifier) @type)\n (#match? @type \"^[A-Z]\"))\n((scoped_type_identifier\n  path: (scoped_identifier\n    name: (identifier) @type))\n (#match? @type \"^[A-Z]\"))\n\n; Assume other uppercase names are enum constructors\n((identifier) @constructor\n (#match? @constructor \"^[A-Z]\"))\n\n; Assume all qualified names in struct patterns are enum constructors. (They're\n; either that, or struct names; highlighting both as constructors seems to be\n; the less glaring choice of error, visually.)\n(struct_pattern\n  type: (scoped_type_identifier\n    name: (type_identifier) @constructor))\n\n; Function calls\n\n(call_expression\n  function: (identifier) @function)\n(call_expression\n  function: (field_expression\n    field: (field_identifier) @function.method))\n(call_expression\n  function: (scoped_identifier\n    \"::\"\n    name: (identifier) @function))\n\n(generic_function\n  function: (identifier) @function)\n(generic_function\n  function: (scoped_identifier\n    name: (identifier) @function))\n(generic_function\n  function: (field_expression\n    field: (field_identifier) @function.method))\n\n(macro_invocation\n  macro: (identifier) @function.macro\n  \"!\" @function.macro)\n\n; Function definitions\n\n(function_item (identifier) @function)\n(function_signature_item (identifier) @function)\n\n; Other identifiers\n\n(type_identifier) @type\n(primitive_type) @type.builtin\n(field_identifier) @property\n\n(line_comment) @comment\n(block_comment) @comment\n\n\"(\" @punctuation.bracket\n\")\" @punctuation.bracket\n\"[\" @punctuation.bracket\n\"]\" @punctuation.bracket\n\"{\" @punctuation.bracket\n\"}\" @punctuation.bracket\n\n(type_arguments\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n(type_parameters\n  \"<\" @punctuation.bracket\n  \">\" @punctuation.bracket)\n\n\"::\" @punctuation.delimiter\n\":\" @punctuation.delimiter\n\".\" @punctuation.delimiter\n\",\" @punctuation.delimiter\n\";\" @punctuation.delimiter\n\n(parameter (identifier) @variable.parameter)\n\n(lifetime (identifier) @label)\n\n\"as\" @keyword\n\"async\" @keyword\n\"await\" @keyword\n\"break\" @keyword\n\"const\" @keyword\n\"continue\" @keyword\n\"default\" @keyword\n\"dyn\" @keyword\n\"else\" @keyword\n\"enum\" @keyword\n\"extern\" @keyword\n\"fn\" @keyword\n\"for\" @keyword\n\"if\" @keyword\n\"impl\" @keyword\n\"in\" @keyword\n\"let\" @keyword\n\"loop\" @keyword\n\"macro_rules!\" @keyword\n\"match\" @keyword\n\"mod\" @keyword\n\"move\" @keyword\n\"pub\" @keyword\n\"ref\" @keyword\n\"return\" @keyword\n\"static\" @keyword\n\"struct\" @keyword\n\"trait\" @keyword\n\"type\" @keyword\n\"union\" @keyword\n\"unsafe\" @keyword\n\"use\" @keyword\n\"where\" @keyword\n\"while\" @keyword\n(crate) @keyword\n(mutable_specifier) @keyword\n(use_list (self) @keyword)\n(scoped_use_list (self) @keyword)\n(scoped_identifier (self) @keyword)\n(super) @keyword\n\n(self) @variable.builtin\n\n(char_literal) @string\n(string_literal) @string\n(raw_string_literal) @string\n\n(boolean_literal) @constant.builtin\n(integer_literal) @constant.builtin\n(float_literal) @constant.builtin\n\n(escape_sequence) @escape\n\n(attribute_item) @attribute\n(inner_attribute_item) @attribute\n\n\"*\" @operator\n\"&\" @operator\n\"'\" @operator\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Highlights__.get_or_init(__Mk__Highlights)
    }
    #[inline]
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        match_: tree_sitter::QueryMatch<'cursor, 'tree>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_, tree }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index as usize { 0usize => Self :: Capture :: Constant { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 1usize => Self :: Capture :: Type { node : < anon_unions :: Type < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 2usize => Self :: Capture :: Constructor { node : < anon_unions :: Constructor < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 3usize => Self :: Capture :: Function { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 4usize => Self :: Capture :: FunctionMethod { node : < super :: nodes :: FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 5usize => Self :: Capture :: FunctionMacro { node : < anon_unions :: FunctionMacro < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 6usize => Self :: Capture :: TypeBuiltin { node : < super :: nodes :: PrimitiveType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 7usize => Self :: Capture :: Property { node : < super :: nodes :: FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 8usize => Self :: Capture :: Comment { node : < anon_unions :: Comment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 9usize => Self :: Capture :: PunctuationBracket { node : < anon_unions :: PunctuationBracket < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 10usize => Self :: Capture :: PunctuationDelimiter { node : < anon_unions :: PunctuationDelimiter < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 11usize => Self :: Capture :: VariableParameter { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 12usize => Self :: Capture :: Label { node : < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 13usize => Self :: Capture :: Keyword { node : < anon_unions :: Keyword < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 14usize => Self :: Capture :: VariableBuiltin { node : < super :: nodes :: _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 15usize => Self :: Capture :: String { node : < anon_unions :: String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 16usize => Self :: Capture :: ConstantBuiltin { node : < anon_unions :: ConstantBuiltin < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 17usize => Self :: Capture :: Escape { node : < super :: nodes :: EscapeSequence < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 18usize => Self :: Capture :: Attribute { node : < anon_unions :: Attribute < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , 19usize => Self :: Capture :: Operator { node : < anon_unions :: Operator < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `constant` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant(&self) -> Option<super::nodes::Identifier<'tree>> {
        { [0u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `type` ([anon_unions::Type])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn r#type(&self) -> Option<anon_unions::Type<'tree>> {
        { [1u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Type < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `constructor` ([anon_unions::Constructor])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constructor(&self) -> Option<anon_unions::Constructor<'tree>> {
        { [2u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Constructor < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function` ([anon_unions::Function])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function(&self) -> Option<super::nodes::Identifier<'tree>> {
        { [3u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function.method` ([anon_unions::FunctionMethod])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_method(&self) -> Option<super::nodes::FieldIdentifier<'tree>> {
        { [4u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `function.macro` ([anon_unions::FunctionMacro])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_macro(&self) -> Option<anon_unions::FunctionMacro<'tree>> {
        { [5u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: FunctionMacro < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `type.builtin` ([super::nodes::PrimitiveType])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_builtin(&self) -> Option<super::nodes::PrimitiveType<'tree>> {
        { [6u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: PrimitiveType < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `property` ([super::nodes::FieldIdentifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn property(&self) -> Option<super::nodes::FieldIdentifier<'tree>> {
        { [7u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `comment` ([anon_unions::Comment])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn comment(&self) -> Option<anon_unions::Comment<'tree>> {
        { [8u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Comment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `punctuation.bracket` ([anon_unions::PunctuationBracket])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn punctuation_bracket(&self) -> Option<anon_unions::PunctuationBracket<'tree>> {
        { [9u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: PunctuationBracket < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `punctuation.delimiter` ([anon_unions::PunctuationDelimiter])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn punctuation_delimiter(&self) -> Option<anon_unions::PunctuationDelimiter<'tree>> {
        {
            [10u32]
                .into_iter()
                .flat_map(|i| self.match_.nodes_for_capture_index(i))
                .map(|n| unsafe {
                    <anon_unions::PunctuationDelimiter<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(
                        type_sitter_lib::tree_sitter_wrapper::Node::new(n, self.tree),
                    )
                })
        }
        .next()
    }
    #[doc = "Returns an iterator over the nodes captured by `variable.parameter` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn variable_parameter(&self) -> Option<super::nodes::Identifier<'tree>> {
        { [11u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `label` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn label(&self) -> Option<super::nodes::Identifier<'tree>> {
        { [12u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `keyword` ([anon_unions::Keyword])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn keyword(&self) -> impl Iterator<Item = anon_unions::Keyword<'tree>> + '_ {
        {
            [13u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Keyword < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) })
        }
    }
    #[doc = "Returns an iterator over the nodes captured by `variable.builtin` ([super::nodes::_Self])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn variable_builtin(&self) -> Option<super::nodes::_Self<'tree>> {
        { [14u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `string` ([anon_unions::String])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(&self) -> Option<anon_unions::String<'tree>> {
        { [15u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: String < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `constant.builtin` ([anon_unions::ConstantBuiltin])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant_builtin(&self) -> Option<anon_unions::ConstantBuiltin<'tree>> {
        { [16u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: ConstantBuiltin < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `escape` ([super::nodes::EscapeSequence])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn escape(&self) -> Option<super::nodes::EscapeSequence<'tree>> {
        { [17u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: EscapeSequence < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `attribute` ([anon_unions::Attribute])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn attribute(&self) -> Option<anon_unions::Attribute<'tree>> {
        { [18u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Attribute < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
    #[doc = "Returns an iterator over the nodes captured by `operator` ([anon_unions::Operator])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn operator(&self) -> Option<anon_unions::Operator<'tree>> {
        { [19u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < anon_unions :: Operator < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next ()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for HighlightsMatch<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(HighlightsMatch))
            .field("match_", &self.match_)
            .finish()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
    for HighlightsMatch<'cursor, 'tree>
{
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Highlights
    }
    #[inline]
    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
        self.tree
    }
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.match_
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.match_
    }
}
#[automatically_derived]
impl<'cursor, 'tree> HighlightsCapture<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `constant` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constant"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Constant { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `type` ([anon_unions::Type])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(type_identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "(identifier) @type"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn r#type(&self) -> Option<&anon_unions::Type<'tree>> {
        match self {
            Self::Type { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `constructor` ([anon_unions::Constructor])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @constructor"]
    #[doc = "(type_identifier) @constructor"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constructor(&self) -> Option<&anon_unions::Constructor<'tree>> {
        match self {
            Self::Constructor { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function` ([anon_unions::Function])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "(identifier) @function"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Function { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function.method` ([anon_unions::FunctionMethod])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "(field_identifier) @function.method"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_method(&self) -> Option<&super::nodes::FieldIdentifier<'tree>> {
        match self {
            Self::FunctionMethod { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `function.macro` ([anon_unions::FunctionMacro])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @function.macro"]
    #[doc = "\"!\" @function.macro"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_macro(&self) -> Option<&anon_unions::FunctionMacro<'tree>> {
        match self {
            Self::FunctionMacro { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `type.builtin` ([super::nodes::PrimitiveType])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(primitive_type) @type.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_builtin(&self) -> Option<&super::nodes::PrimitiveType<'tree>> {
        match self {
            Self::TypeBuiltin { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `property` ([super::nodes::FieldIdentifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(field_identifier) @property"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn property(&self) -> Option<&super::nodes::FieldIdentifier<'tree>> {
        match self {
            Self::Property { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `comment` ([anon_unions::Comment])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(line_comment) @comment"]
    #[doc = "(block_comment) @comment"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn comment(&self) -> Option<&anon_unions::Comment<'tree>> {
        match self {
            Self::Comment { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.bracket` ([anon_unions::PunctuationBracket])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"(\" @punctuation.bracket"]
    #[doc = "\")\" @punctuation.bracket"]
    #[doc = "\"[\" @punctuation.bracket"]
    #[doc = "\"]\" @punctuation.bracket"]
    #[doc = "\"{\" @punctuation.bracket"]
    #[doc = "\"}\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "\"<\" @punctuation.bracket"]
    #[doc = "\">\" @punctuation.bracket"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn punctuation_bracket(&self) -> Option<&anon_unions::PunctuationBracket<'tree>> {
        match self {
            Self::PunctuationBracket { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `punctuation.delimiter` ([anon_unions::PunctuationDelimiter])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"::\" @punctuation.delimiter"]
    #[doc = "\":\" @punctuation.delimiter"]
    #[doc = "\".\" @punctuation.delimiter"]
    #[doc = "\",\" @punctuation.delimiter"]
    #[doc = "\";\" @punctuation.delimiter"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn punctuation_delimiter(&self) -> Option<&anon_unions::PunctuationDelimiter<'tree>> {
        match self {
            Self::PunctuationDelimiter { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `variable.parameter` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @variable.parameter"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn variable_parameter(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::VariableParameter { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `label` ([super::nodes::Identifier])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(identifier) @label"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn label(&self) -> Option<&super::nodes::Identifier<'tree>> {
        match self {
            Self::Label { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `keyword` ([anon_unions::Keyword])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"as\" @keyword"]
    #[doc = "\"async\" @keyword"]
    #[doc = "\"await\" @keyword"]
    #[doc = "\"break\" @keyword"]
    #[doc = "\"const\" @keyword"]
    #[doc = "\"continue\" @keyword"]
    #[doc = "\"default\" @keyword"]
    #[doc = "\"dyn\" @keyword"]
    #[doc = "\"else\" @keyword"]
    #[doc = "\"enum\" @keyword"]
    #[doc = "\"extern\" @keyword"]
    #[doc = "\"fn\" @keyword"]
    #[doc = "\"for\" @keyword"]
    #[doc = "\"if\" @keyword"]
    #[doc = "\"impl\" @keyword"]
    #[doc = "\"in\" @keyword"]
    #[doc = "\"let\" @keyword"]
    #[doc = "\"loop\" @keyword"]
    #[doc = "\"macro_rules!\" @keyword"]
    #[doc = "\"match\" @keyword"]
    #[doc = "\"mod\" @keyword"]
    #[doc = "\"move\" @keyword"]
    #[doc = "\"pub\" @keyword"]
    #[doc = "\"ref\" @keyword"]
    #[doc = "\"return\" @keyword"]
    #[doc = "\"static\" @keyword"]
    #[doc = "\"struct\" @keyword"]
    #[doc = "\"trait\" @keyword"]
    #[doc = "\"type\" @keyword"]
    #[doc = "\"union\" @keyword"]
    #[doc = "\"unsafe\" @keyword"]
    #[doc = "\"use\" @keyword"]
    #[doc = "\"where\" @keyword"]
    #[doc = "\"while\" @keyword"]
    #[doc = "(crate) @keyword"]
    #[doc = "(mutable_specifier) @keyword"]
    #[doc = "(super) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "(self) @keyword"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn keyword(&self) -> Option<&anon_unions::Keyword<'tree>> {
        match self {
            Self::Keyword { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `variable.builtin` ([super::nodes::_Self])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(self) @variable.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn variable_builtin(&self) -> Option<&super::nodes::_Self<'tree>> {
        match self {
            Self::VariableBuiltin { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `string` ([anon_unions::String])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(char_literal) @string"]
    #[doc = "(string_literal) @string"]
    #[doc = "(raw_string_literal) @string"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string(&self) -> Option<&anon_unions::String<'tree>> {
        match self {
            Self::String { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `constant.builtin` ([anon_unions::ConstantBuiltin])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(boolean_literal) @constant.builtin"]
    #[doc = "(integer_literal) @constant.builtin"]
    #[doc = "(float_literal) @constant.builtin"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn constant_builtin(&self) -> Option<&anon_unions::ConstantBuiltin<'tree>> {
        match self {
            Self::ConstantBuiltin { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `escape` ([super::nodes::EscapeSequence])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(escape_sequence) @escape"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn escape(&self) -> Option<&super::nodes::EscapeSequence<'tree>> {
        match self {
            Self::Escape { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `attribute` ([anon_unions::Attribute])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(attribute_item) @attribute"]
    #[doc = "(inner_attribute_item) @attribute"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn attribute(&self) -> Option<&anon_unions::Attribute<'tree>> {
        match self {
            Self::Attribute { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
    #[doc = "Try to interpret this capture as a `operator` ([anon_unions::Operator])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "\"*\" @operator"]
    #[doc = "\"&\" @operator"]
    #[doc = "\"'\" @operator"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn operator(&self) -> Option<&anon_unions::Operator<'tree>> {
        match self {
            Self::Operator { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for HighlightsCapture<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Constant)
                ))
                .field("node", node)
                .finish(),
            Self::Type { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Type)
                ))
                .field("node", node)
                .finish(),
            Self::Constructor { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Constructor)
                ))
                .field("node", node)
                .finish(),
            Self::Function { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Function)
                ))
                .field("node", node)
                .finish(),
            Self::FunctionMethod { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(FunctionMethod)
                ))
                .field("node", node)
                .finish(),
            Self::FunctionMacro { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(FunctionMacro)
                ))
                .field("node", node)
                .finish(),
            Self::TypeBuiltin { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(TypeBuiltin)
                ))
                .field("node", node)
                .finish(),
            Self::Property { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Property)
                ))
                .field("node", node)
                .finish(),
            Self::Comment { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Comment)
                ))
                .field("node", node)
                .finish(),
            Self::PunctuationBracket { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(PunctuationBracket)
                ))
                .field("node", node)
                .finish(),
            Self::PunctuationDelimiter { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(PunctuationDelimiter)
                ))
                .field("node", node)
                .finish(),
            Self::VariableParameter { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(VariableParameter)
                ))
                .field("node", node)
                .finish(),
            Self::Label { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Label)
                ))
                .field("node", node)
                .finish(),
            Self::Keyword { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Keyword)
                ))
                .field("node", node)
                .finish(),
            Self::VariableBuiltin { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(VariableBuiltin)
                ))
                .field("node", node)
                .finish(),
            Self::String { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(String)
                ))
                .field("node", node)
                .finish(),
            Self::ConstantBuiltin { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(ConstantBuiltin)
                ))
                .field("node", node)
                .finish(),
            Self::Escape { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Escape)
                ))
                .field("node", node)
                .finish(),
            Self::Attribute { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Attribute)
                ))
                .field("node", node)
                .finish(),
            Self::Operator { node, .. } => f
                .debug_struct(concat!(
                    stringify!(HighlightsCapture),
                    "::",
                    stringify!(Operator)
                ))
                .field("node", node)
                .finish(),
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for HighlightsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::Constant { node, .. } => Self::Constant {
                node: *node,
                match_: None,
            },
            Self::Type { node, .. } => Self::Type {
                node: *node,
                match_: None,
            },
            Self::Constructor { node, .. } => Self::Constructor {
                node: *node,
                match_: None,
            },
            Self::Function { node, .. } => Self::Function {
                node: *node,
                match_: None,
            },
            Self::FunctionMethod { node, .. } => Self::FunctionMethod {
                node: *node,
                match_: None,
            },
            Self::FunctionMacro { node, .. } => Self::FunctionMacro {
                node: *node,
                match_: None,
            },
            Self::TypeBuiltin { node, .. } => Self::TypeBuiltin {
                node: *node,
                match_: None,
            },
            Self::Property { node, .. } => Self::Property {
                node: *node,
                match_: None,
            },
            Self::Comment { node, .. } => Self::Comment {
                node: *node,
                match_: None,
            },
            Self::PunctuationBracket { node, .. } => Self::PunctuationBracket {
                node: *node,
                match_: None,
            },
            Self::PunctuationDelimiter { node, .. } => Self::PunctuationDelimiter {
                node: *node,
                match_: None,
            },
            Self::VariableParameter { node, .. } => Self::VariableParameter {
                node: *node,
                match_: None,
            },
            Self::Label { node, .. } => Self::Label {
                node: *node,
                match_: None,
            },
            Self::Keyword { node, .. } => Self::Keyword {
                node: *node,
                match_: None,
            },
            Self::VariableBuiltin { node, .. } => Self::VariableBuiltin {
                node: *node,
                match_: None,
            },
            Self::String { node, .. } => Self::String {
                node: *node,
                match_: None,
            },
            Self::ConstantBuiltin { node, .. } => Self::ConstantBuiltin {
                node: *node,
                match_: None,
            },
            Self::Escape { node, .. } => Self::Escape {
                node: *node,
                match_: None,
            },
            Self::Attribute { node, .. } => Self::Attribute {
                node: *node,
                match_: None,
            },
            Self::Operator { node, .. } => Self::Operator {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for HighlightsCapture<'cursor, 'tree>
{
    type Query = Highlights;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Highlights
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Constant { match_, .. } => match_.as_ref(),
            Self::Type { match_, .. } => match_.as_ref(),
            Self::Constructor { match_, .. } => match_.as_ref(),
            Self::Function { match_, .. } => match_.as_ref(),
            Self::FunctionMethod { match_, .. } => match_.as_ref(),
            Self::FunctionMacro { match_, .. } => match_.as_ref(),
            Self::TypeBuiltin { match_, .. } => match_.as_ref(),
            Self::Property { match_, .. } => match_.as_ref(),
            Self::Comment { match_, .. } => match_.as_ref(),
            Self::PunctuationBracket { match_, .. } => match_.as_ref(),
            Self::PunctuationDelimiter { match_, .. } => match_.as_ref(),
            Self::VariableParameter { match_, .. } => match_.as_ref(),
            Self::Label { match_, .. } => match_.as_ref(),
            Self::Keyword { match_, .. } => match_.as_ref(),
            Self::VariableBuiltin { match_, .. } => match_.as_ref(),
            Self::String { match_, .. } => match_.as_ref(),
            Self::ConstantBuiltin { match_, .. } => match_.as_ref(),
            Self::Escape { match_, .. } => match_.as_ref(),
            Self::Attribute { match_, .. } => match_.as_ref(),
            Self::Operator { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::Constant { match_, .. } => match_,
            Self::Type { match_, .. } => match_,
            Self::Constructor { match_, .. } => match_,
            Self::Function { match_, .. } => match_,
            Self::FunctionMethod { match_, .. } => match_,
            Self::FunctionMacro { match_, .. } => match_,
            Self::TypeBuiltin { match_, .. } => match_,
            Self::Property { match_, .. } => match_,
            Self::Comment { match_, .. } => match_,
            Self::PunctuationBracket { match_, .. } => match_,
            Self::PunctuationDelimiter { match_, .. } => match_,
            Self::VariableParameter { match_, .. } => match_,
            Self::Label { match_, .. } => match_,
            Self::Keyword { match_, .. } => match_,
            Self::VariableBuiltin { match_, .. } => match_,
            Self::String { match_, .. } => match_,
            Self::ConstantBuiltin { match_, .. } => match_,
            Self::Escape { match_, .. } => match_,
            Self::Attribute { match_, .. } => match_,
            Self::Operator { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(&self) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 0usize,
                name: "constant",
            },
            Self::Type { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 1usize,
                name: "type",
            },
            Self::Constructor { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 2usize,
                name: "constructor",
            },
            Self::Function { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 3usize,
                name: "function",
            },
            Self::FunctionMethod { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 4usize,
                    name: "function.method",
                }
            }
            Self::FunctionMacro { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 5usize,
                    name: "function.macro",
                }
            }
            Self::TypeBuiltin { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 6usize,
                name: "type.builtin",
            },
            Self::Property { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 7usize,
                name: "property",
            },
            Self::Comment { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 8usize,
                name: "comment",
            },
            Self::PunctuationBracket { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 9usize,
                    name: "punctuation.bracket",
                }
            }
            Self::PunctuationDelimiter { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 10usize,
                    name: "punctuation.delimiter",
                }
            }
            Self::VariableParameter { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 11usize,
                    name: "variable.parameter",
                }
            }
            Self::Label { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 12usize,
                name: "label",
            },
            Self::Keyword { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 13usize,
                name: "keyword",
            },
            Self::VariableBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 14usize,
                    name: "variable.builtin",
                }
            }
            Self::String { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 15usize,
                name: "string",
            },
            Self::ConstantBuiltin { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 16usize,
                    name: "constant.builtin",
                }
            }
            Self::Escape { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 17usize,
                name: "escape",
            },
            Self::Attribute { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 18usize,
                name: "attribute",
            },
            Self::Operator { node, .. } => type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                node: *node.node(),
                index: 19usize,
                name: "operator",
            },
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => node.node(),
            Self::Type { node, .. } => node.node(),
            Self::Constructor { node, .. } => node.node(),
            Self::Function { node, .. } => node.node(),
            Self::FunctionMethod { node, .. } => node.node(),
            Self::FunctionMacro { node, .. } => node.node(),
            Self::TypeBuiltin { node, .. } => node.node(),
            Self::Property { node, .. } => node.node(),
            Self::Comment { node, .. } => node.node(),
            Self::PunctuationBracket { node, .. } => node.node(),
            Self::PunctuationDelimiter { node, .. } => node.node(),
            Self::VariableParameter { node, .. } => node.node(),
            Self::Label { node, .. } => node.node(),
            Self::Keyword { node, .. } => node.node(),
            Self::VariableBuiltin { node, .. } => node.node(),
            Self::String { node, .. } => node.node(),
            Self::ConstantBuiltin { node, .. } => node.node(),
            Self::Escape { node, .. } => node.node(),
            Self::Attribute { node, .. } => node.node(),
            Self::Operator { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::Constant { node, .. } => node.node_mut(),
            Self::Type { node, .. } => node.node_mut(),
            Self::Constructor { node, .. } => node.node_mut(),
            Self::Function { node, .. } => node.node_mut(),
            Self::FunctionMethod { node, .. } => node.node_mut(),
            Self::FunctionMacro { node, .. } => node.node_mut(),
            Self::TypeBuiltin { node, .. } => node.node_mut(),
            Self::Property { node, .. } => node.node_mut(),
            Self::Comment { node, .. } => node.node_mut(),
            Self::PunctuationBracket { node, .. } => node.node_mut(),
            Self::PunctuationDelimiter { node, .. } => node.node_mut(),
            Self::VariableParameter { node, .. } => node.node_mut(),
            Self::Label { node, .. } => node.node_mut(),
            Self::Keyword { node, .. } => node.node_mut(),
            Self::VariableBuiltin { node, .. } => node.node_mut(),
            Self::String { node, .. } => node.node_mut(),
            Self::ConstantBuiltin { node, .. } => node.node_mut(),
            Self::Escape { node, .. } => node.node_mut(),
            Self::Attribute { node, .. } => node.node_mut(),
            Self::Operator { node, .. } => node.node_mut(),
        }
    }
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::Constant { .. } => "constant",
            Self::Type { .. } => "type",
            Self::Constructor { .. } => "constructor",
            Self::Function { .. } => "function",
            Self::FunctionMethod { .. } => "function.method",
            Self::FunctionMacro { .. } => "function.macro",
            Self::TypeBuiltin { .. } => "type.builtin",
            Self::Property { .. } => "property",
            Self::Comment { .. } => "comment",
            Self::PunctuationBracket { .. } => "punctuation.bracket",
            Self::PunctuationDelimiter { .. } => "punctuation.delimiter",
            Self::VariableParameter { .. } => "variable.parameter",
            Self::Label { .. } => "label",
            Self::Keyword { .. } => "keyword",
            Self::VariableBuiltin { .. } => "variable.builtin",
            Self::String { .. } => "string",
            Self::ConstantBuiltin { .. } => "constant.builtin",
            Self::Escape { .. } => "escape",
            Self::Attribute { .. } => "attribute",
            Self::Operator { .. } => "operator",
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::Constant { .. } => 0usize,
            Self::Type { .. } => 1usize,
            Self::Constructor { .. } => 2usize,
            Self::Function { .. } => 3usize,
            Self::FunctionMethod { .. } => 4usize,
            Self::FunctionMacro { .. } => 5usize,
            Self::TypeBuiltin { .. } => 6usize,
            Self::Property { .. } => 7usize,
            Self::Comment { .. } => 8usize,
            Self::PunctuationBracket { .. } => 9usize,
            Self::PunctuationDelimiter { .. } => 10usize,
            Self::VariableParameter { .. } => 11usize,
            Self::Label { .. } => 12usize,
            Self::Keyword { .. } => 13usize,
            Self::VariableBuiltin { .. } => 14usize,
            Self::String { .. } => 15usize,
            Self::ConstantBuiltin { .. } => 16usize,
            Self::Escape { .. } => 17usize,
            Self::Attribute { .. } => 18usize,
            Self::Operator { .. } => 19usize,
        }
    }
}
#[allow(non_upper_case_globals)]
static __Injections__: type_sitter_lib::gen_internal::TypedQueryOnceBox<tree_sitter::Query> =
    type_sitter_lib::gen_internal::TypedQueryOnceBox::new();
#[allow(non_snake_case)]
fn __Mk__Injections() -> Box<tree_sitter::Query> {
    # [allow (unused_mut)] let mut query = tree_sitter :: Query :: new (tree_sitter_rust :: language () , "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n") . expect ("query parsed at compile-time but failed at runtime. Is the language 'tree_sitter_rust' correct, and did you use the same tree-sitter / tree_sitter_rust version?") ;
    Box::new(query)
}
#[doc = "Typed version of the query:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub struct Injections;
#[doc = "Matches returned by a query cursor running the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type InjectionsMatches<'cursor, 'tree> =
    type_sitter_lib::TypedQueryMatches<'cursor, 'tree, Injections>;
#[doc = "Captures returned by a query cursor running the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
#[allow(unused, non_camel_case_types)]
pub type InjectionsCaptures<'cursor, 'tree> =
    type_sitter_lib::TypedQueryCaptures<'cursor, 'tree, Injections>;
#[doc = "A match returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
pub struct InjectionsMatch<'cursor, 'tree> {
    match_: tree_sitter::QueryMatch<'cursor, 'tree>,
    tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
}
#[doc = "A capture returned by the query [Injections]:\n\n```sexp\n((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n```"]
pub enum InjectionsCapture<'cursor, 'tree> {
    #[doc = "A `injection.content` ([anon_unions::InjectionContent])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    InjectionContent {
        node: super::nodes::TokenTree<'tree>,
        match_: Option<InjectionsMatch<'cursor, 'tree>>,
    },
}
#[automatically_derived]
impl type_sitter_lib::TypedQuery for Injections {
    type Match<'cursor, 'tree: 'cursor> = InjectionsMatch<'cursor, 'tree>;
    type Capture<'cursor, 'tree: 'cursor> = InjectionsCapture<'cursor, 'tree>;
    fn query_str(&self) -> &'static str {
        "((macro_invocation\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n\n((macro_rule\n  (token_tree) @injection.content)\n (#set! injection.language \"rust\")\n (#set! injection.include-children))\n"
    }
    fn query(&self) -> &'static tree_sitter::Query {
        __Injections__.get_or_init(__Mk__Injections)
    }
    #[inline]
    unsafe fn wrap_match<'cursor, 'tree>(
        &self,
        match_: tree_sitter::QueryMatch<'cursor, 'tree>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Match<'cursor, 'tree> {
        Self::Match { match_, tree }
    }
    #[inline]
    unsafe fn wrap_capture<'cursor, 'tree>(
        &self,
        capture: tree_sitter::QueryCapture<'tree>,
        match_: Option<Self::Match<'cursor, 'tree>>,
        tree: &'tree type_sitter_lib::tree_sitter_wrapper::Tree,
    ) -> Self::Capture<'cursor, 'tree> {
        match capture . index as usize { 0usize => Self :: Capture :: InjectionContent { node : < super :: nodes :: TokenTree < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (unsafe { type_sitter_lib :: tree_sitter_wrapper :: Node :: new (capture . node , tree) }) , match_ } , capture_index => unreachable ! ("Invalid capture index: {}" , capture_index) }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> InjectionsMatch<'cursor, 'tree> {
    #[doc = "Returns an iterator over the nodes captured by `injection.content` ([anon_unions::InjectionContent])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn injection_content(&self) -> super::nodes::TokenTree<'tree> {
        let result = { [0u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TokenTree < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next () . expect ("one quantifier returned nothing") ;
        debug_assert ! ({ [0u32] . into_iter () . flat_map (| i | self . match_ . nodes_for_capture_index (i)) . map (| n | unsafe { < super :: nodes :: TokenTree < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (type_sitter_lib :: tree_sitter_wrapper :: Node :: new (n , self . tree)) }) } . next () . is_none () , "one quantifier returned more than one item");
        result
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for InjectionsMatch<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(InjectionsMatch))
            .field("match_", &self.match_)
            .finish()
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryMatch<'cursor, 'tree>
    for InjectionsMatch<'cursor, 'tree>
{
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Injections
    }
    #[inline]
    fn tree(&self) -> &'tree type_sitter_lib::tree_sitter_wrapper::Tree {
        self.tree
    }
    #[inline]
    fn raw(&self) -> &tree_sitter::QueryMatch<'cursor, 'tree> {
        &self.match_
    }
    #[inline]
    fn into_raw(self) -> tree_sitter::QueryMatch<'cursor, 'tree> {
        self.match_
    }
}
#[automatically_derived]
impl<'cursor, 'tree> InjectionsCapture<'cursor, 'tree> {
    #[doc = "Try to interpret this capture as a `injection.content` ([anon_unions::InjectionContent])"]
    #[doc = ""]
    #[doc = "The full capture including pattern(s) is:"]
    #[doc = "```sexp"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "(token_tree) @injection.content"]
    #[doc = "```"]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn injection_content(&self) -> Option<&super::nodes::TokenTree<'tree>> {
        match self {
            Self::InjectionContent { node, .. } => Some(node),
            #[allow(unreachable_patterns)]
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> std::fmt::Debug for InjectionsCapture<'cursor, 'tree> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InjectionContent { node, .. } => f
                .debug_struct(concat!(
                    stringify!(InjectionsCapture),
                    "::",
                    stringify!(InjectionContent)
                ))
                .field("node", node)
                .finish(),
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> Clone for InjectionsCapture<'cursor, 'tree> {
    fn clone(&self) -> Self {
        match self {
            Self::InjectionContent { node, .. } => Self::InjectionContent {
                node: *node,
                match_: None,
            },
        }
    }
}
#[automatically_derived]
impl<'cursor, 'tree> type_sitter_lib::TypedQueryCapture<'cursor, 'tree>
    for InjectionsCapture<'cursor, 'tree>
{
    type Query = Injections;
    #[inline]
    fn query(&self) -> &'cursor Self::Query {
        &Injections
    }
    #[inline]
    fn match_(
        &self,
    ) -> Option<&<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::InjectionContent { match_, .. } => match_.as_ref(),
        }
    }
    #[inline]
    fn into_match(
        self,
    ) -> Option<<Self::Query as type_sitter_lib::TypedQuery>::Match<'cursor, 'tree>> {
        match self {
            Self::InjectionContent { match_, .. } => match_,
        }
    }
    #[inline]
    fn to_raw(&self) -> type_sitter_lib::tree_sitter_wrapper::QueryCapture<'static, 'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => {
                type_sitter_lib::tree_sitter_wrapper::QueryCapture {
                    node: *node.node(),
                    index: 0usize,
                    name: "injection.content",
                }
            }
        }
    }
    #[inline]
    fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node(),
        }
    }
    #[inline]
    fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
        use type_sitter_lib::TypedNode;
        match self {
            Self::InjectionContent { node, .. } => node.node_mut(),
        }
    }
    #[inline]
    fn name(&self) -> &'static str {
        match self {
            Self::InjectionContent { .. } => "injection.content",
        }
    }
    #[inline]
    fn index(&self) -> usize {
        match self {
            Self::InjectionContent { .. } => 0usize,
        }
    }
}
pub mod anon_unions {
    #[allow(unused_imports)]
    use super::super::nodes::*;
    #[doc = "one of `{field_identifier | identifier | type_identifier}`:\n- [FieldIdentifier]\n- [Identifier]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Name<'tree> {
        FieldIdentifier(FieldIdentifier<'tree>),
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Name<'tree> {
        #[doc = "Returns the node if it is of kind `field_identifier` ([FieldIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn field_identifier(self) -> Option<FieldIdentifier<'tree>> {
            match self {
                Self::FieldIdentifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Name<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "field_identifier" => {
                    Ok(unsafe {
                        Self :: FieldIdentifier (< FieldIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Name<'tree> {
        const KIND: &'static str = "{field_identifier | identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node(),
                Self::Identifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::FieldIdentifier(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
                Self::TypeIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{enum_item | struct_item | type_item | union_item}`:\n- [EnumItem]\n- [StructItem]\n- [TypeItem]\n- [UnionItem]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum DefinitionClass<'tree> {
        EnumItem(EnumItem<'tree>),
        StructItem(StructItem<'tree>),
        TypeItem(TypeItem<'tree>),
        UnionItem(UnionItem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> DefinitionClass<'tree> {
        #[doc = "Returns the node if it is of kind `enum_item` ([EnumItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn enum_item(self) -> Option<EnumItem<'tree>> {
            match self {
                Self::EnumItem(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `struct_item` ([StructItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn struct_item(self) -> Option<StructItem<'tree>> {
            match self {
                Self::StructItem(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_item` ([TypeItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_item(self) -> Option<TypeItem<'tree>> {
            match self {
                Self::TypeItem(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `union_item` ([UnionItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn union_item(self) -> Option<UnionItem<'tree>> {
            match self {
                Self::UnionItem(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for DefinitionClass<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "enum_item" => {
                    Ok(unsafe {
                        Self :: EnumItem (< EnumItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "struct_item" => Ok(unsafe {
                    Self :: StructItem (< StructItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_item" => {
                    Ok(unsafe {
                        Self :: TypeItem (< TypeItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "union_item" => Ok(unsafe {
                    Self :: UnionItem (< UnionItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for DefinitionClass<'tree> {
        const KIND: &'static str = "{enum_item | struct_item | type_item | union_item}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::EnumItem(x) => x.node(),
                Self::StructItem(x) => x.node(),
                Self::TypeItem(x) => x.node(),
                Self::UnionItem(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::EnumItem(x) => x.node_mut(),
                Self::StructItem(x) => x.node_mut(),
                Self::TypeItem(x) => x.node_mut(),
                Self::UnionItem(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::EnumItem(x) => x.into_node(),
                Self::StructItem(x) => x.into_node(),
                Self::TypeItem(x) => x.into_node(),
                Self::UnionItem(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{call_expression | macro_invocation}`:\n- [CallExpression]\n- [MacroInvocation]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ReferenceCall<'tree> {
        CallExpression(CallExpression<'tree>),
        MacroInvocation(MacroInvocation<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ReferenceCall<'tree> {
        #[doc = "Returns the node if it is of kind `call_expression` ([CallExpression]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn call_expression(self) -> Option<CallExpression<'tree>> {
            match self {
                Self::CallExpression(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `macro_invocation` ([MacroInvocation]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
            match self {
                Self::MacroInvocation(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for ReferenceCall<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "call_expression" => {
                    Ok(unsafe {
                        Self :: CallExpression (< CallExpression < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "macro_invocation" => {
                    Ok(unsafe {
                        Self :: MacroInvocation (< MacroInvocation < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferenceCall<'tree> {
        const KIND: &'static str = "{call_expression | macro_invocation}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CallExpression(x) => x.node(),
                Self::MacroInvocation(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CallExpression(x) => x.node_mut(),
                Self::MacroInvocation(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CallExpression(x) => x.into_node(),
                Self::MacroInvocation(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{identifier | type_identifier}`:\n- [Identifier]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Type<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Type<'tree> {
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Type<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_node(),
                Self::TypeIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{identifier | type_identifier}`:\n- [Identifier]\n- [TypeIdentifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Constructor<'tree> {
        Identifier(Identifier<'tree>),
        TypeIdentifier(TypeIdentifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Constructor<'tree> {
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type_identifier` ([TypeIdentifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
            match self {
                Self::TypeIdentifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Constructor<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "type_identifier" => {
                    Ok(unsafe {
                        Self :: TypeIdentifier (< TypeIdentifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Constructor<'tree> {
        const KIND: &'static str = "{identifier | type_identifier}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node(),
                Self::TypeIdentifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.node_mut(),
                Self::TypeIdentifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Identifier(x) => x.into_node(),
                Self::TypeIdentifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{! | identifier}`:\n- [symbols::Not]\n- [Identifier]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum FunctionMacro<'tree> {
        Not(symbols::Not<'tree>),
        Identifier(Identifier<'tree>),
    }
    #[automatically_derived]
    impl<'tree> FunctionMacro<'tree> {
        #[doc = "Returns the node if it is of kind `!` ([symbols::Not]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn not(self) -> Option<symbols::Not<'tree>> {
            match self {
                Self::Not(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `identifier` ([Identifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn identifier(self) -> Option<Identifier<'tree>> {
            match self {
                Self::Identifier(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for FunctionMacro<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "!" => Ok(unsafe {
                    Self :: Not (< symbols :: Not < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "identifier" => Ok(unsafe {
                    Self :: Identifier (< Identifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionMacro<'tree> {
        const KIND: &'static str = "{! | identifier}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Not(x) => x.node(),
                Self::Identifier(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Not(x) => x.node_mut(),
                Self::Identifier(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Not(x) => x.into_node(),
                Self::Identifier(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{block_comment | line_comment}`:\n- [BlockComment]\n- [LineComment]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Comment<'tree> {
        BlockComment(BlockComment<'tree>),
        LineComment(LineComment<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Comment<'tree> {
        #[doc = "Returns the node if it is of kind `block_comment` ([BlockComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn block_comment(self) -> Option<BlockComment<'tree>> {
            match self {
                Self::BlockComment(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `line_comment` ([LineComment]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn line_comment(self) -> Option<LineComment<'tree>> {
            match self {
                Self::LineComment(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Comment<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "block_comment" => Ok(unsafe {
                    Self :: BlockComment (< BlockComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "line_comment" => Ok(unsafe {
                    Self :: LineComment (< LineComment < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Comment<'tree> {
        const KIND: &'static str = "{block_comment | line_comment}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.node(),
                Self::LineComment(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.node_mut(),
                Self::LineComment(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BlockComment(x) => x.into_node(),
                Self::LineComment(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{( | ) | < | > | [ | ] | { | }}`:\n- [symbols::LParen]\n- [symbols::RParen]\n- [symbols::Lt]\n- [symbols::Gt]\n- [symbols::LBracket]\n- [symbols::RBracket]\n- [symbols::LBrace]\n- [symbols::RBrace]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PunctuationBracket<'tree> {
        LParen(symbols::LParen<'tree>),
        RParen(symbols::RParen<'tree>),
        Lt(symbols::Lt<'tree>),
        Gt(symbols::Gt<'tree>),
        LBracket(symbols::LBracket<'tree>),
        RBracket(symbols::RBracket<'tree>),
        LBrace(symbols::LBrace<'tree>),
        RBrace(symbols::RBrace<'tree>),
    }
    #[automatically_derived]
    impl<'tree> PunctuationBracket<'tree> {
        #[doc = "Returns the node if it is of kind `(` ([symbols::LParen]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_paren(self) -> Option<symbols::LParen<'tree>> {
            match self {
                Self::LParen(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `)` ([symbols::RParen]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_paren(self) -> Option<symbols::RParen<'tree>> {
            match self {
                Self::RParen(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `<` ([symbols::Lt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn lt(self) -> Option<symbols::Lt<'tree>> {
            match self {
                Self::Lt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `>` ([symbols::Gt]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn gt(self) -> Option<symbols::Gt<'tree>> {
            match self {
                Self::Gt(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `[` ([symbols::LBracket]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_bracket(self) -> Option<symbols::LBracket<'tree>> {
            match self {
                Self::LBracket(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `]` ([symbols::RBracket]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_bracket(self) -> Option<symbols::RBracket<'tree>> {
            match self {
                Self::RBracket(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `{` ([symbols::LBrace]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn l_brace(self) -> Option<symbols::LBrace<'tree>> {
            match self {
                Self::LBrace(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `}` ([symbols::RBrace]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r_brace(self) -> Option<symbols::RBrace<'tree>> {
            match self {
                Self::RBrace(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
        for PunctuationBracket<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "(" => Ok(unsafe {
                    Self::LParen(<symbols::LParen<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                ")" => Ok(unsafe {
                    Self::RParen(<symbols::RParen<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "<" => Ok(unsafe {
                    Self :: Lt (< symbols :: Lt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                ">" => Ok(unsafe {
                    Self :: Gt (< symbols :: Gt < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "[" => Ok(unsafe {
                    Self::LBracket(<symbols::LBracket<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "]" => Ok(unsafe {
                    Self::RBracket(<symbols::RBracket<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "{" => Ok(unsafe {
                    Self::LBrace(<symbols::LBrace<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "}" => Ok(unsafe {
                    Self::RBrace(<symbols::RBrace<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for PunctuationBracket<'tree> {
        const KIND: &'static str = "{( | ) | < | > | [ | ] | { | }}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::LParen(x) => x.node(),
                Self::RParen(x) => x.node(),
                Self::Lt(x) => x.node(),
                Self::Gt(x) => x.node(),
                Self::LBracket(x) => x.node(),
                Self::RBracket(x) => x.node(),
                Self::LBrace(x) => x.node(),
                Self::RBrace(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::LParen(x) => x.node_mut(),
                Self::RParen(x) => x.node_mut(),
                Self::Lt(x) => x.node_mut(),
                Self::Gt(x) => x.node_mut(),
                Self::LBracket(x) => x.node_mut(),
                Self::RBracket(x) => x.node_mut(),
                Self::LBrace(x) => x.node_mut(),
                Self::RBrace(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::LParen(x) => x.into_node(),
                Self::RParen(x) => x.into_node(),
                Self::Lt(x) => x.into_node(),
                Self::Gt(x) => x.into_node(),
                Self::LBracket(x) => x.into_node(),
                Self::RBracket(x) => x.into_node(),
                Self::LBrace(x) => x.into_node(),
                Self::RBrace(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{, | . | : | :: | ;}`:\n- [symbols::Comma]\n- [symbols::Dot]\n- [symbols::Colon]\n- [symbols::ColonColon]\n- [symbols::Semicolon]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum PunctuationDelimiter<'tree> {
        Comma(symbols::Comma<'tree>),
        Dot(symbols::Dot<'tree>),
        Colon(symbols::Colon<'tree>),
        ColonColon(symbols::ColonColon<'tree>),
        Semicolon(symbols::Semicolon<'tree>),
    }
    #[automatically_derived]
    impl<'tree> PunctuationDelimiter<'tree> {
        #[doc = "Returns the node if it is of kind `,` ([symbols::Comma]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn comma(self) -> Option<symbols::Comma<'tree>> {
            match self {
                Self::Comma(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `.` ([symbols::Dot]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn dot(self) -> Option<symbols::Dot<'tree>> {
            match self {
                Self::Dot(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `:` ([symbols::Colon]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn colon(self) -> Option<symbols::Colon<'tree>> {
            match self {
                Self::Colon(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `::` ([symbols::ColonColon]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn colon_colon(self) -> Option<symbols::ColonColon<'tree>> {
            match self {
                Self::ColonColon(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `;` ([symbols::Semicolon]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn semicolon(self) -> Option<symbols::Semicolon<'tree>> {
            match self {
                Self::Semicolon(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>>
        for PunctuationDelimiter<'tree>
    {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "," => Ok(unsafe {
                    Self::Comma(<symbols::Comma<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "." => Ok(unsafe {
                    Self :: Dot (< symbols :: Dot < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                ":" => Ok(unsafe {
                    Self::Colon(<symbols::Colon<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "::" => {
                    Ok(unsafe {
                        Self :: ColonColon (< symbols :: ColonColon < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                ";" => Ok(unsafe {
                    Self::Semicolon(<symbols::Semicolon<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for PunctuationDelimiter<'tree> {
        const KIND: &'static str = "{, | . | : | :: | ;}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Comma(x) => x.node(),
                Self::Dot(x) => x.node(),
                Self::Colon(x) => x.node(),
                Self::ColonColon(x) => x.node(),
                Self::Semicolon(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Comma(x) => x.node_mut(),
                Self::Dot(x) => x.node_mut(),
                Self::Colon(x) => x.node_mut(),
                Self::ColonColon(x) => x.node_mut(),
                Self::Semicolon(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::Comma(x) => x.into_node(),
                Self::Dot(x) => x.into_node(),
                Self::Colon(x) => x.into_node(),
                Self::ColonColon(x) => x.into_node(),
                Self::Semicolon(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while}`:\n- [unnamed::As]\n- [unnamed::Async]\n- [unnamed::Await]\n- [unnamed::Break]\n- [unnamed::Const]\n- [unnamed::Continue]\n- [Crate]\n- [unnamed::Default]\n- [unnamed::Dyn]\n- [unnamed::Else]\n- [unnamed::Enum]\n- [unnamed::Extern]\n- [unnamed::Fn]\n- [unnamed::For]\n- [unnamed::If]\n- [unnamed::Impl]\n- [unnamed::In]\n- [unnamed::Let]\n- [unnamed::Loop]\n- [symbols::MacroRulesNot]\n- [unnamed::Match]\n- [unnamed::Mod]\n- [unnamed::Move]\n- [MutableSpecifier]\n- [unnamed::Pub]\n- [unnamed::Ref]\n- [unnamed::Return]\n- [_Self]\n- [unnamed::Static]\n- [unnamed::Struct]\n- [Super]\n- [unnamed::Trait]\n- [unnamed::Type]\n- [unnamed::Union]\n- [unnamed::Unsafe]\n- [unnamed::Use]\n- [unnamed::Where]\n- [unnamed::While]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Keyword<'tree> {
        As(unnamed::As<'tree>),
        Async(unnamed::Async<'tree>),
        Await(unnamed::Await<'tree>),
        Break(unnamed::Break<'tree>),
        Const(unnamed::Const<'tree>),
        Continue(unnamed::Continue<'tree>),
        Crate(Crate<'tree>),
        Default(unnamed::Default<'tree>),
        Dyn(unnamed::Dyn<'tree>),
        Else(unnamed::Else<'tree>),
        Enum(unnamed::Enum<'tree>),
        Extern(unnamed::Extern<'tree>),
        Fn(unnamed::Fn<'tree>),
        For(unnamed::For<'tree>),
        If(unnamed::If<'tree>),
        Impl(unnamed::Impl<'tree>),
        In(unnamed::In<'tree>),
        Let(unnamed::Let<'tree>),
        Loop(unnamed::Loop<'tree>),
        MacroRulesNot(symbols::MacroRulesNot<'tree>),
        Match(unnamed::Match<'tree>),
        Mod(unnamed::Mod<'tree>),
        Move(unnamed::Move<'tree>),
        MutableSpecifier(MutableSpecifier<'tree>),
        Pub(unnamed::Pub<'tree>),
        Ref(unnamed::Ref<'tree>),
        Return(unnamed::Return<'tree>),
        _Self(_Self<'tree>),
        Static(unnamed::Static<'tree>),
        Struct(unnamed::Struct<'tree>),
        Super(Super<'tree>),
        Trait(unnamed::Trait<'tree>),
        Type(unnamed::Type<'tree>),
        Union(unnamed::Union<'tree>),
        Unsafe(unnamed::Unsafe<'tree>),
        Use(unnamed::Use<'tree>),
        Where(unnamed::Where<'tree>),
        While(unnamed::While<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Keyword<'tree> {
        #[doc = "Returns the node if it is of kind `as` ([unnamed::As]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#as(self) -> Option<unnamed::As<'tree>> {
            match self {
                Self::As(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `async` ([unnamed::Async]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#async(self) -> Option<unnamed::Async<'tree>> {
            match self {
                Self::Async(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `await` ([unnamed::Await]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#await(self) -> Option<unnamed::Await<'tree>> {
            match self {
                Self::Await(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `break` ([unnamed::Break]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#break(self) -> Option<unnamed::Break<'tree>> {
            match self {
                Self::Break(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `const` ([unnamed::Const]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#const(self) -> Option<unnamed::Const<'tree>> {
            match self {
                Self::Const(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `continue` ([unnamed::Continue]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#continue(self) -> Option<unnamed::Continue<'tree>> {
            match self {
                Self::Continue(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `crate` ([Crate]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn crate_(self) -> Option<Crate<'tree>> {
            match self {
                Self::Crate(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `default` ([unnamed::Default]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn default(self) -> Option<unnamed::Default<'tree>> {
            match self {
                Self::Default(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `dyn` ([unnamed::Dyn]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#dyn(self) -> Option<unnamed::Dyn<'tree>> {
            match self {
                Self::Dyn(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `else` ([unnamed::Else]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#else(self) -> Option<unnamed::Else<'tree>> {
            match self {
                Self::Else(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `enum` ([unnamed::Enum]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#enum(self) -> Option<unnamed::Enum<'tree>> {
            match self {
                Self::Enum(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `extern` ([unnamed::Extern]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#extern(self) -> Option<unnamed::Extern<'tree>> {
            match self {
                Self::Extern(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `fn` ([unnamed::Fn]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#fn(self) -> Option<unnamed::Fn<'tree>> {
            match self {
                Self::Fn(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `for` ([unnamed::For]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#for(self) -> Option<unnamed::For<'tree>> {
            match self {
                Self::For(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `if` ([unnamed::If]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#if(self) -> Option<unnamed::If<'tree>> {
            match self {
                Self::If(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `impl` ([unnamed::Impl]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#impl(self) -> Option<unnamed::Impl<'tree>> {
            match self {
                Self::Impl(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `in` ([unnamed::In]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#in(self) -> Option<unnamed::In<'tree>> {
            match self {
                Self::In(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `let` ([unnamed::Let]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#let(self) -> Option<unnamed::Let<'tree>> {
            match self {
                Self::Let(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `loop` ([unnamed::Loop]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#loop(self) -> Option<unnamed::Loop<'tree>> {
            match self {
                Self::Loop(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `macro_rules!` ([symbols::MacroRulesNot]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn macro_rules_not(self) -> Option<symbols::MacroRulesNot<'tree>> {
            match self {
                Self::MacroRulesNot(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `match` ([unnamed::Match]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#match(self) -> Option<unnamed::Match<'tree>> {
            match self {
                Self::Match(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `mod` ([unnamed::Mod]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#mod(self) -> Option<unnamed::Mod<'tree>> {
            match self {
                Self::Mod(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `move` ([unnamed::Move]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#move(self) -> Option<unnamed::Move<'tree>> {
            match self {
                Self::Move(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `mutable_specifier` ([MutableSpecifier]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mutable_specifier(self) -> Option<MutableSpecifier<'tree>> {
            match self {
                Self::MutableSpecifier(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `pub` ([unnamed::Pub]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#pub(self) -> Option<unnamed::Pub<'tree>> {
            match self {
                Self::Pub(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `ref` ([unnamed::Ref]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#ref(self) -> Option<unnamed::Ref<'tree>> {
            match self {
                Self::Ref(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `return` ([unnamed::Return]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#return(self) -> Option<unnamed::Return<'tree>> {
            match self {
                Self::Return(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `self` ([_Self]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn _self(self) -> Option<_Self<'tree>> {
            match self {
                Self::_Self(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `static` ([unnamed::Static]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#static(self) -> Option<unnamed::Static<'tree>> {
            match self {
                Self::Static(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `struct` ([unnamed::Struct]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#struct(self) -> Option<unnamed::Struct<'tree>> {
            match self {
                Self::Struct(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `super` ([Super]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn super_(self) -> Option<Super<'tree>> {
            match self {
                Self::Super(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `trait` ([unnamed::Trait]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#trait(self) -> Option<unnamed::Trait<'tree>> {
            match self {
                Self::Trait(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `type` ([unnamed::Type]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#type(self) -> Option<unnamed::Type<'tree>> {
            match self {
                Self::Type(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `union` ([unnamed::Union]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn union(self) -> Option<unnamed::Union<'tree>> {
            match self {
                Self::Union(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `unsafe` ([unnamed::Unsafe]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#unsafe(self) -> Option<unnamed::Unsafe<'tree>> {
            match self {
                Self::Unsafe(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `use` ([unnamed::Use]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#use(self) -> Option<unnamed::Use<'tree>> {
            match self {
                Self::Use(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `where` ([unnamed::Where]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#where(self) -> Option<unnamed::Where<'tree>> {
            match self {
                Self::Where(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `while` ([unnamed::While]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn r#while(self) -> Option<unnamed::While<'tree>> {
            match self {
                Self::While(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Keyword<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "as" => Ok(unsafe {
                    Self :: As (< unnamed :: As < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "async" => Ok(unsafe {
                    Self::Async(<unnamed::Async<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "await" => Ok(unsafe {
                    Self::Await(<unnamed::Await<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "break" => Ok(unsafe {
                    Self::Break(<unnamed::Break<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "const" => Ok(unsafe {
                    Self::Const(<unnamed::Const<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "continue" => Ok(unsafe {
                    Self::Continue(<unnamed::Continue<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "crate" => {
                    Ok(unsafe {
                        Self :: Crate (< Crate < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "default" => Ok(unsafe {
                    Self::Default(<unnamed::Default<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "dyn" => Ok(unsafe {
                    Self :: Dyn (< unnamed :: Dyn < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "else" => Ok(unsafe {
                    Self::Else(<unnamed::Else<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "enum" => Ok(unsafe {
                    Self::Enum(<unnamed::Enum<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "extern" => Ok(unsafe {
                    Self::Extern(<unnamed::Extern<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "fn" => Ok(unsafe {
                    Self :: Fn (< unnamed :: Fn < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "for" => Ok(unsafe {
                    Self :: For (< unnamed :: For < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "if" => Ok(unsafe {
                    Self :: If (< unnamed :: If < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "impl" => Ok(unsafe {
                    Self::Impl(<unnamed::Impl<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "in" => Ok(unsafe {
                    Self :: In (< unnamed :: In < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "let" => Ok(unsafe {
                    Self :: Let (< unnamed :: Let < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "loop" => Ok(unsafe {
                    Self::Loop(<unnamed::Loop<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "macro_rules!" => Ok(unsafe {
                    Self :: MacroRulesNot (< symbols :: MacroRulesNot < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "match" => Ok(unsafe {
                    Self::Match(<unnamed::Match<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "mod" => Ok(unsafe {
                    Self :: Mod (< unnamed :: Mod < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "move" => Ok(unsafe {
                    Self::Move(<unnamed::Move<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "mutable_specifier" => Ok(unsafe {
                    Self :: MutableSpecifier (< MutableSpecifier < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "pub" => Ok(unsafe {
                    Self :: Pub (< unnamed :: Pub < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "ref" => Ok(unsafe {
                    Self :: Ref (< unnamed :: Ref < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "return" => Ok(unsafe {
                    Self::Return(<unnamed::Return<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "self" => {
                    Ok(unsafe {
                        Self :: _Self (< _Self < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "static" => Ok(unsafe {
                    Self::Static(<unnamed::Static<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "struct" => Ok(unsafe {
                    Self::Struct(<unnamed::Struct<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "super" => {
                    Ok(unsafe {
                        Self :: Super (< Super < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "trait" => Ok(unsafe {
                    Self::Trait(<unnamed::Trait<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "type" => Ok(unsafe {
                    Self::Type(<unnamed::Type<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "union" => Ok(unsafe {
                    Self::Union(<unnamed::Union<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "unsafe" => Ok(unsafe {
                    Self::Unsafe(<unnamed::Unsafe<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "use" => Ok(unsafe {
                    Self :: Use (< unnamed :: Use < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "where" => Ok(unsafe {
                    Self::Where(<unnamed::Where<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "while" => Ok(unsafe {
                    Self::While(<unnamed::While<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Keyword<'tree> {
        const KIND : & 'static str = "{as | async | await | break | const | continue | crate | default | dyn | else | enum | extern | fn | for | if | impl | in | let | loop | macro_rules! | match | mod | move | mutable_specifier | pub | ref | return | self | static | struct | super | trait | type | union | unsafe | use | where | while}" ;
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::As(x) => x.node(),
                Self::Async(x) => x.node(),
                Self::Await(x) => x.node(),
                Self::Break(x) => x.node(),
                Self::Const(x) => x.node(),
                Self::Continue(x) => x.node(),
                Self::Crate(x) => x.node(),
                Self::Default(x) => x.node(),
                Self::Dyn(x) => x.node(),
                Self::Else(x) => x.node(),
                Self::Enum(x) => x.node(),
                Self::Extern(x) => x.node(),
                Self::Fn(x) => x.node(),
                Self::For(x) => x.node(),
                Self::If(x) => x.node(),
                Self::Impl(x) => x.node(),
                Self::In(x) => x.node(),
                Self::Let(x) => x.node(),
                Self::Loop(x) => x.node(),
                Self::MacroRulesNot(x) => x.node(),
                Self::Match(x) => x.node(),
                Self::Mod(x) => x.node(),
                Self::Move(x) => x.node(),
                Self::MutableSpecifier(x) => x.node(),
                Self::Pub(x) => x.node(),
                Self::Ref(x) => x.node(),
                Self::Return(x) => x.node(),
                Self::_Self(x) => x.node(),
                Self::Static(x) => x.node(),
                Self::Struct(x) => x.node(),
                Self::Super(x) => x.node(),
                Self::Trait(x) => x.node(),
                Self::Type(x) => x.node(),
                Self::Union(x) => x.node(),
                Self::Unsafe(x) => x.node(),
                Self::Use(x) => x.node(),
                Self::Where(x) => x.node(),
                Self::While(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::As(x) => x.node_mut(),
                Self::Async(x) => x.node_mut(),
                Self::Await(x) => x.node_mut(),
                Self::Break(x) => x.node_mut(),
                Self::Const(x) => x.node_mut(),
                Self::Continue(x) => x.node_mut(),
                Self::Crate(x) => x.node_mut(),
                Self::Default(x) => x.node_mut(),
                Self::Dyn(x) => x.node_mut(),
                Self::Else(x) => x.node_mut(),
                Self::Enum(x) => x.node_mut(),
                Self::Extern(x) => x.node_mut(),
                Self::Fn(x) => x.node_mut(),
                Self::For(x) => x.node_mut(),
                Self::If(x) => x.node_mut(),
                Self::Impl(x) => x.node_mut(),
                Self::In(x) => x.node_mut(),
                Self::Let(x) => x.node_mut(),
                Self::Loop(x) => x.node_mut(),
                Self::MacroRulesNot(x) => x.node_mut(),
                Self::Match(x) => x.node_mut(),
                Self::Mod(x) => x.node_mut(),
                Self::Move(x) => x.node_mut(),
                Self::MutableSpecifier(x) => x.node_mut(),
                Self::Pub(x) => x.node_mut(),
                Self::Ref(x) => x.node_mut(),
                Self::Return(x) => x.node_mut(),
                Self::_Self(x) => x.node_mut(),
                Self::Static(x) => x.node_mut(),
                Self::Struct(x) => x.node_mut(),
                Self::Super(x) => x.node_mut(),
                Self::Trait(x) => x.node_mut(),
                Self::Type(x) => x.node_mut(),
                Self::Union(x) => x.node_mut(),
                Self::Unsafe(x) => x.node_mut(),
                Self::Use(x) => x.node_mut(),
                Self::Where(x) => x.node_mut(),
                Self::While(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::As(x) => x.into_node(),
                Self::Async(x) => x.into_node(),
                Self::Await(x) => x.into_node(),
                Self::Break(x) => x.into_node(),
                Self::Const(x) => x.into_node(),
                Self::Continue(x) => x.into_node(),
                Self::Crate(x) => x.into_node(),
                Self::Default(x) => x.into_node(),
                Self::Dyn(x) => x.into_node(),
                Self::Else(x) => x.into_node(),
                Self::Enum(x) => x.into_node(),
                Self::Extern(x) => x.into_node(),
                Self::Fn(x) => x.into_node(),
                Self::For(x) => x.into_node(),
                Self::If(x) => x.into_node(),
                Self::Impl(x) => x.into_node(),
                Self::In(x) => x.into_node(),
                Self::Let(x) => x.into_node(),
                Self::Loop(x) => x.into_node(),
                Self::MacroRulesNot(x) => x.into_node(),
                Self::Match(x) => x.into_node(),
                Self::Mod(x) => x.into_node(),
                Self::Move(x) => x.into_node(),
                Self::MutableSpecifier(x) => x.into_node(),
                Self::Pub(x) => x.into_node(),
                Self::Ref(x) => x.into_node(),
                Self::Return(x) => x.into_node(),
                Self::_Self(x) => x.into_node(),
                Self::Static(x) => x.into_node(),
                Self::Struct(x) => x.into_node(),
                Self::Super(x) => x.into_node(),
                Self::Trait(x) => x.into_node(),
                Self::Type(x) => x.into_node(),
                Self::Union(x) => x.into_node(),
                Self::Unsafe(x) => x.into_node(),
                Self::Use(x) => x.into_node(),
                Self::Where(x) => x.into_node(),
                Self::While(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{char_literal | raw_string_literal | string_literal}`:\n- [CharLiteral]\n- [RawStringLiteral]\n- [StringLiteral]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum String<'tree> {
        CharLiteral(CharLiteral<'tree>),
        RawStringLiteral(RawStringLiteral<'tree>),
        StringLiteral(StringLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> String<'tree> {
        #[doc = "Returns the node if it is of kind `char_literal` ([CharLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn char_literal(self) -> Option<CharLiteral<'tree>> {
            match self {
                Self::CharLiteral(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `raw_string_literal` ([RawStringLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
            match self {
                Self::RawStringLiteral(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `string_literal` ([StringLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn string_literal(self) -> Option<StringLiteral<'tree>> {
            match self {
                Self::StringLiteral(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for String<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "char_literal" => Ok(unsafe {
                    Self :: CharLiteral (< CharLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "raw_string_literal" => Ok(unsafe {
                    Self :: RawStringLiteral (< RawStringLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "string_literal" => Ok(unsafe {
                    Self::StringLiteral(<StringLiteral<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for String<'tree> {
        const KIND: &'static str = "{char_literal | raw_string_literal | string_literal}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.node(),
                Self::RawStringLiteral(x) => x.node(),
                Self::StringLiteral(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.node_mut(),
                Self::RawStringLiteral(x) => x.node_mut(),
                Self::StringLiteral(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::CharLiteral(x) => x.into_node(),
                Self::RawStringLiteral(x) => x.into_node(),
                Self::StringLiteral(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{boolean_literal | float_literal | integer_literal}`:\n- [BooleanLiteral]\n- [FloatLiteral]\n- [IntegerLiteral]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum ConstantBuiltin<'tree> {
        BooleanLiteral(BooleanLiteral<'tree>),
        FloatLiteral(FloatLiteral<'tree>),
        IntegerLiteral(IntegerLiteral<'tree>),
    }
    #[automatically_derived]
    impl<'tree> ConstantBuiltin<'tree> {
        #[doc = "Returns the node if it is of kind `boolean_literal` ([BooleanLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
            match self {
                Self::BooleanLiteral(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `float_literal` ([FloatLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
            match self {
                Self::FloatLiteral(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `integer_literal` ([IntegerLiteral]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
            match self {
                Self::IntegerLiteral(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for ConstantBuiltin<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "boolean_literal" => {
                    Ok(unsafe {
                        Self :: BooleanLiteral (< BooleanLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                "float_literal" => Ok(unsafe {
                    Self :: FloatLiteral (< FloatLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "integer_literal" => {
                    Ok(unsafe {
                        Self :: IntegerLiteral (< IntegerLiteral < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                    })
                }
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstantBuiltin<'tree> {
        const KIND: &'static str = "{boolean_literal | float_literal | integer_literal}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => x.node(),
                Self::FloatLiteral(x) => x.node(),
                Self::IntegerLiteral(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => x.node_mut(),
                Self::FloatLiteral(x) => x.node_mut(),
                Self::IntegerLiteral(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::BooleanLiteral(x) => x.into_node(),
                Self::FloatLiteral(x) => x.into_node(),
                Self::IntegerLiteral(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{attribute_item | inner_attribute_item}`:\n- [AttributeItem]\n- [InnerAttributeItem]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Attribute<'tree> {
        AttributeItem(AttributeItem<'tree>),
        InnerAttributeItem(InnerAttributeItem<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Attribute<'tree> {
        #[doc = "Returns the node if it is of kind `attribute_item` ([AttributeItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
            match self {
                Self::AttributeItem(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `inner_attribute_item` ([InnerAttributeItem]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn inner_attribute_item(self) -> Option<InnerAttributeItem<'tree>> {
            match self {
                Self::InnerAttributeItem(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Attribute<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "attribute_item" => Ok(unsafe {
                    Self::AttributeItem(<AttributeItem<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "inner_attribute_item" => Ok(unsafe {
                    Self :: InnerAttributeItem (< InnerAttributeItem < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Attribute<'tree> {
        const KIND: &'static str = "{attribute_item | inner_attribute_item}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node(),
                Self::InnerAttributeItem(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.node_mut(),
                Self::InnerAttributeItem(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::AttributeItem(x) => x.into_node(),
                Self::InnerAttributeItem(x) => x.into_node(),
            }
        }
    }
    #[doc = "one of `{& | ' | *}`:\n- [symbols::And]\n- [symbols::Quote]\n- [symbols::Mul]"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types)]
    pub enum Operator<'tree> {
        And(symbols::And<'tree>),
        Quote(symbols::Quote<'tree>),
        Mul(symbols::Mul<'tree>),
    }
    #[automatically_derived]
    impl<'tree> Operator<'tree> {
        #[doc = "Returns the node if it is of kind `&` ([symbols::And]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn and(self) -> Option<symbols::And<'tree>> {
            match self {
                Self::And(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `'` ([symbols::Quote]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn quote(self) -> Option<symbols::Quote<'tree>> {
            match self {
                Self::Quote(x) => Some(x),
                _ => None,
            }
        }
        #[doc = "Returns the node if it is of kind `*` ([symbols::Mul]), otherwise returns None"]
        #[inline]
        #[allow(unused, non_snake_case)]
        pub fn mul(self) -> Option<symbols::Mul<'tree>> {
            match self {
                Self::Mul(x) => Some(x),
                _ => None,
            }
        }
    }
    #[automatically_derived]
    impl<'tree> TryFrom<type_sitter_lib::tree_sitter_wrapper::Node<'tree>> for Operator<'tree> {
        type Error = type_sitter_lib::IncorrectKind<'tree>;
        #[inline]
        fn try_from(
            node: type_sitter_lib::tree_sitter_wrapper::Node<'tree>,
        ) -> Result<Self, Self::Error> {
            match node.kind() {
                "&" => Ok(unsafe {
                    Self :: And (< symbols :: And < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                "'" => Ok(unsafe {
                    Self::Quote(<symbols::Quote<'tree> as type_sitter_lib::TypedNode<
                        'tree,
                    >>::from_node_unchecked(node))
                }),
                "*" => Ok(unsafe {
                    Self :: Mul (< symbols :: Mul < 'tree > as type_sitter_lib :: TypedNode < 'tree >> :: from_node_unchecked (node))
                }),
                _ => Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                }),
            }
        }
    }
    #[automatically_derived]
    impl<'tree> type_sitter_lib::TypedNode<'tree> for Operator<'tree> {
        const KIND: &'static str = "{& | ' | *}";
        #[inline]
        fn node(&self) -> &type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::And(x) => x.node(),
                Self::Quote(x) => x.node(),
                Self::Mul(x) => x.node(),
            }
        }
        #[inline]
        fn node_mut(&mut self) -> &mut type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::And(x) => x.node_mut(),
                Self::Quote(x) => x.node_mut(),
                Self::Mul(x) => x.node_mut(),
            }
        }
        #[inline]
        fn into_node(self) -> type_sitter_lib::tree_sitter_wrapper::Node<'tree> {
            match self {
                Self::And(x) => x.into_node(),
                Self::Quote(x) => x.into_node(),
                Self::Mul(x) => x.into_node(),
            }
        }
    }
}
