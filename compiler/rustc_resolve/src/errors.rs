use rustc_errors::{codes::*, Applicability, MultiSpan};
use rustc_macros::{Diagnostic, Subdiagnostic};
use rustc_span::{
    symbol::{Ident, Symbol},
    Span,
};

use crate::{late::PatternSource, Res};

#[derive(Diagnostic)]
#[diag(resolve_parent_module_reset_for_binding, code = E0637)]
pub(crate) struct ParentModuleResetForBinding;

#[derive(Diagnostic)]
#[diag(resolve_ampersand_used_without_explicit_lifetime_name, code = E0637)]
#[note]
pub(crate) struct AmpersandUsedWithoutExplicitLifetimeName(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_underscore_lifetime_name_cannot_be_used_here, code = E0637)]
#[note]
pub(crate) struct UnderscoreLifetimeNameCannotBeUsedHere(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_crate_may_not_be_imported)]
pub(crate) struct CrateMayNotBeImported(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_crate_root_imports_must_be_named_explicitly)]
pub(crate) struct CrateRootNamesMustBeNamedExplicitly(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_crate_root_imports_must_be_named_explicitly)]
pub(crate) struct ResolutionError(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_generic_params_from_outer_item, code = E0401)]
pub(crate) struct GenericParamsFromOuterItem {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) label: Option<GenericParamsFromOuterItemLabel>,
    #[label(resolve_refer_to_type_directly)]
    pub(crate) refer_to_type_directly: Option<Span>,
    #[subdiagnostic]
    pub(crate) sugg: Option<GenericParamsFromOuterItemSugg>,
    #[subdiagnostic]
    pub(crate) static_or_const: Option<GenericParamsFromOuterItemStaticOrConst>,
    pub(crate) is_self: bool,
}

#[derive(Subdiagnostic)]
pub(crate) enum GenericParamsFromOuterItemStaticOrConst {
    #[note(resolve_generic_params_from_outer_item_static)]
    Static,
    #[note(resolve_generic_params_from_outer_item_const)]
    Const,
}

#[derive(Subdiagnostic)]
pub(crate) enum GenericParamsFromOuterItemLabel {
    #[label(resolve_generic_params_from_outer_item_self_ty_param)]
    SelfTyParam(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_self_ty_alias)]
    SelfTyAlias(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_ty_param)]
    TyParam(#[primary_span] Span),
    #[label(resolve_generic_params_from_outer_item_const_param)]
    ConstParam(#[primary_span] Span),
}

#[derive(Subdiagnostic)]
#[suggestion(resolve_suggestion, code = "{snippet}", applicability = "maybe-incorrect")]
pub(crate) struct GenericParamsFromOuterItemSugg {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) snippet: String,
}

#[derive(Diagnostic)]
#[diag(resolve_name_is_already_used_as_generic_parameter, code = E0403)]
pub(crate) struct NameAlreadyUsedInParameterList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_first_use_of_name)]
    pub(crate) first_use_span: Span,
    pub(crate) name: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_method_not_member_of_trait, code = E0407)]
pub(crate) struct MethodNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) method: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedFnWithSimilarNameExists>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_fn_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedFnWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_type_not_member_of_trait, code = E0437)]
pub(crate) struct TypeNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) type_: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedTypeWithSimilarNameExists>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_type_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedTypeWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_const_not_member_of_trait, code = E0438)]
pub(crate) struct ConstNotMemberOfTrait {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) const_: Ident,
    pub(crate) trait_: String,
    #[subdiagnostic]
    pub(crate) sub: Option<AssociatedConstWithSimilarNameExists>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_associated_const_with_similar_name_exists,
    code = "{candidate}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AssociatedConstWithSimilarNameExists {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) candidate: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_variable_bound_with_different_mode, code = E0409)]
pub(crate) struct VariableBoundWithDifferentMode {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_first_binding_span)]
    pub(crate) first_binding_span: Span,
    pub(crate) variable_name: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_ident_bound_more_than_once_in_parameter_list, code = E0415)]
pub(crate) struct IdentifierBoundMoreThanOnceInParameterList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) identifier: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_ident_bound_more_than_once_in_same_pattern, code = E0416)]
pub(crate) struct IdentifierBoundMoreThanOnceInSamePattern {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) identifier: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_undeclared_label, code = E0426)]
pub(crate) struct UndeclaredLabel {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) sub_reachable: Option<LabelWithSimilarNameReachable>,
    #[subdiagnostic]
    pub(crate) sub_reachable_suggestion: Option<TryUsingSimilarlyNamedLabel>,
    #[subdiagnostic]
    pub(crate) sub_unreachable: Option<UnreachableLabelWithSimilarNameExists>,
}

#[derive(Subdiagnostic)]
#[label(resolve_label_with_similar_name_reachable)]
pub(crate) struct LabelWithSimilarNameReachable(#[primary_span] pub(crate) Span);

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_try_using_similarly_named_label,
    code = "{ident_name}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct TryUsingSimilarlyNamedLabel {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident_name: Symbol,
}

#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_with_similar_name_exists)]
pub(crate) struct UnreachableLabelWithSimilarNameExists {
    #[primary_span]
    pub(crate) ident_span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_self_import_can_only_appear_once_in_the_list, code = E0430)]
pub(crate) struct SelfImportCanOnlyAppearOnceInTheList {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_self_import_only_in_import_list_with_non_empty_prefix, code = E0431)]
pub(crate) struct SelfImportOnlyInImportListWithNonEmptyPrefix {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_capture_dynamic_environment_in_fn_item, code = E0434)]
#[help]
pub(crate) struct CannotCaptureDynamicEnvironmentInFnItem {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_attempt_to_use_non_constant_value_in_constant, code = E0435)]
pub(crate) struct AttemptToUseNonConstantValueInConstant<'a> {
    #[primary_span]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) with: Option<AttemptToUseNonConstantValueInConstantWithSuggestion<'a>>,
    #[subdiagnostic]
    pub(crate) with_label: Option<AttemptToUseNonConstantValueInConstantLabelWithSuggestion>,
    #[subdiagnostic]
    pub(crate) without: Option<AttemptToUseNonConstantValueInConstantWithoutSuggestion<'a>>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_attempt_to_use_non_constant_value_in_constant_with_suggestion,
    code = "{suggestion} {ident}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct AttemptToUseNonConstantValueInConstantWithSuggestion<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) suggestion: &'a str,
    pub(crate) current: &'a str,
}

#[derive(Subdiagnostic)]
#[label(resolve_attempt_to_use_non_constant_value_in_constant_label_with_suggestion)]
pub(crate) struct AttemptToUseNonConstantValueInConstantLabelWithSuggestion {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[label(resolve_attempt_to_use_non_constant_value_in_constant_without_suggestion)]
pub(crate) struct AttemptToUseNonConstantValueInConstantWithoutSuggestion<'a> {
    #[primary_span]
    pub(crate) ident_span: Span,
    pub(crate) suggestion: &'a str,
}

#[derive(Diagnostic)]
#[diag(resolve_self_imports_only_allowed_within, code = E0429)]
pub(crate) struct SelfImportsOnlyAllowedWithin {
    #[primary_span]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) suggestion: Option<SelfImportsOnlyAllowedWithinSuggestion>,
    #[subdiagnostic]
    pub(crate) mpart_suggestion: Option<SelfImportsOnlyAllowedWithinMultipartSuggestion>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_self_imports_only_allowed_within_suggestion,
    code = "",
    applicability = "machine-applicable"
)]
pub(crate) struct SelfImportsOnlyAllowedWithinSuggestion {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_self_imports_only_allowed_within_multipart_suggestion,
    applicability = "machine-applicable"
)]
pub(crate) struct SelfImportsOnlyAllowedWithinMultipartSuggestion {
    #[suggestion_part(code = "{{")]
    pub(crate) multipart_start: Span,
    #[suggestion_part(code = "}}")]
    pub(crate) multipart_end: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_binding_shadows_something_unacceptable, code = E0530)]
pub(crate) struct BindingShadowsSomethingUnacceptable<'a> {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) shadowing_binding: PatternSource,
    pub(crate) shadowed_binding: Res,
    pub(crate) article: &'a str,
    #[subdiagnostic]
    pub(crate) sub_suggestion: Option<BindingShadowsSomethingUnacceptableSuggestion>,
    #[label(resolve_label_shadowed_binding)]
    pub(crate) shadowed_binding_span: Span,
    pub(crate) participle: &'a str,
    pub(crate) name: Symbol,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_binding_shadows_something_unacceptable_suggestion,
    code = "{name}(..)",
    applicability = "unspecified"
)]
pub(crate) struct BindingShadowsSomethingUnacceptableSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_forward_declared_generic_param, code = E0128)]
pub(crate) struct ForwardDeclaredGenericParam {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_param_in_ty_of_const_param, code = E0770)]
pub(crate) struct ParamInTyOfConstParam {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) param_kind: Option<ParamKindInTyOfConstParam>,
}

#[derive(Debug)]
#[derive(Subdiagnostic)]
pub(crate) enum ParamKindInTyOfConstParam {
    #[note(resolve_type_param_in_ty_of_const_param)]
    Type,
    #[note(resolve_const_param_in_ty_of_const_param)]
    Const,
    #[note(resolve_lifetime_param_in_ty_of_const_param)]
    Lifetime,
}

#[derive(Diagnostic)]
#[diag(resolve_self_in_generic_param_default, code = E0735)]
pub(crate) struct SelfInGenericParamDefault {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_param_in_non_trivial_anon_const)]
pub(crate) struct ParamInNonTrivialAnonConst {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) param_kind: ParamKindInNonTrivialAnonConst,
    #[subdiagnostic]
    pub(crate) help: Option<ParamInNonTrivialAnonConstHelp>,
}

#[derive(Subdiagnostic)]
#[help(resolve_param_in_non_trivial_anon_const_help)]
pub(crate) struct ParamInNonTrivialAnonConstHelp;

#[derive(Debug)]
#[derive(Subdiagnostic)]
pub(crate) enum ParamKindInNonTrivialAnonConst {
    #[note(resolve_type_param_in_non_trivial_anon_const)]
    Type,
    #[help(resolve_const_param_in_non_trivial_anon_const)]
    Const { name: Symbol },
    #[note(resolve_lifetime_param_in_non_trivial_anon_const)]
    Lifetime,
}

#[derive(Diagnostic)]
#[diag(resolve_unreachable_label, code = E0767)]
#[note]
pub(crate) struct UnreachableLabel {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[label(resolve_label_definition_span)]
    pub(crate) definition_span: Span,
    #[subdiagnostic]
    pub(crate) sub_suggestion: Option<UnreachableLabelSubSuggestion>,
    #[subdiagnostic]
    pub(crate) sub_suggestion_label: Option<UnreachableLabelSubLabel>,
    #[subdiagnostic]
    pub(crate) sub_unreachable_label: Option<UnreachableLabelSubLabelUnreachable>,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unreachable_label_suggestion_use_similarly_named,
    code = "{ident_name}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct UnreachableLabelSubSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident_name: Symbol,
}

#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_similar_name_reachable)]
pub(crate) struct UnreachableLabelSubLabel {
    #[primary_span]
    pub(crate) ident_span: Span,
}

#[derive(Subdiagnostic)]
#[label(resolve_unreachable_label_similar_name_unreachable)]
pub(crate) struct UnreachableLabelSubLabelUnreachable {
    #[primary_span]
    pub(crate) ident_span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_trait_impl_mismatch)]
pub(crate) struct TraitImplMismatch {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    pub(crate) kind: String,
    #[label(resolve_label_trait_item)]
    pub(crate) trait_item_span: Span,
    pub(crate) trait_path: String,
}

#[derive(Diagnostic)]
#[diag(resolve_invalid_asm_sym)]
#[help]
pub(crate) struct InvalidAsmSym {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_lowercase_self)]
pub(crate) struct LowercaseSelf {
    #[primary_span]
    #[suggestion(code = "Self", applicability = "maybe-incorrect", style = "short")]
    pub(crate) span: Span,
}

#[derive(Debug)]
#[derive(Diagnostic)]
#[diag(resolve_binding_in_never_pattern)]
pub(crate) struct BindingInNeverPattern {
    #[primary_span]
    #[suggestion(code = "_", applicability = "machine-applicable", style = "short")]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_trait_impl_duplicate, code = E0201)]
pub(crate) struct TraitImplDuplicate {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[label(resolve_old_span_label)]
    pub(crate) old_span: Span,
    #[label(resolve_trait_item_span)]
    pub(crate) trait_item_span: Span,
    pub(crate) name: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_relative_2018)]
pub(crate) struct Relative2018 {
    #[primary_span]
    pub(crate) span: Span,
    #[suggestion(code = "crate::{path_str}", applicability = "maybe-incorrect")]
    pub(crate) path_span: Span,
    pub(crate) path_str: String,
}

#[derive(Diagnostic)]
#[diag(resolve_ancestor_only, code = E0742)]
pub(crate) struct AncestorOnly(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_expected_module_found, code = E0577)]
pub(crate) struct ExpectedFound {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) res: Res,
    pub(crate) path_str: String,
}

#[derive(Diagnostic)]
#[diag(resolve_indeterminate, code = E0578)]
pub(crate) struct Indeterminate(#[primary_span] pub(crate) Span);

#[derive(Diagnostic)]
#[diag(resolve_tool_module_imported)]
pub(crate) struct ToolModuleImported {
    #[primary_span]
    pub(crate) span: Span,
    #[note]
    pub(crate) import: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_module_only)]
pub(crate) struct ModuleOnly(#[primary_span] pub(crate) Span);

#[derive(Diagnostic, Default)]
#[diag(resolve_macro_expected_found)]
pub(crate) struct MacroExpectedFound<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) found: &'static str,
    pub(crate) article: &'static str,
    pub(crate) expected: &'static str,
    pub(crate) macro_path: &'a str,
    #[subdiagnostic]
    pub(crate) remove_surrounding_derive: Option<RemoveSurroundingDerive>,
    #[subdiagnostic]
    pub(crate) add_as_non_derive: Option<AddAsNonDerive<'a>>,
}

#[derive(Subdiagnostic)]
#[help(resolve_remove_surrounding_derive)]
pub(crate) struct RemoveSurroundingDerive {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[help(resolve_add_as_non_derive)]
pub(crate) struct AddAsNonDerive<'a> {
    pub(crate) macro_path: &'a str,
}

#[derive(Diagnostic)]
#[diag(resolve_proc_macro_same_crate)]
pub(crate) struct ProcMacroSameCrate {
    #[primary_span]
    pub(crate) span: Span,
    #[help]
    pub(crate) is_test: bool,
}

#[derive(Diagnostic)]
#[diag(resolve_imported_crate)]
pub(crate) struct CrateImported {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_macro_use_extern_crate_self)]
pub(crate) struct MacroUseExternCrateSelf {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_accessible_unsure)]
#[note]
pub(crate) struct CfgAccessibleUnsure {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Debug)]
#[derive(Diagnostic)]
#[diag(resolve_param_in_enum_discriminant)]
pub(crate) struct ParamInEnumDiscriminant {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
    #[subdiagnostic]
    pub(crate) param_kind: ParamKindInEnumDiscriminant,
}

#[derive(Debug)]
#[derive(Subdiagnostic)]
pub(crate) enum ParamKindInEnumDiscriminant {
    #[note(resolve_type_param_in_enum_discriminant)]
    Type,
    #[note(resolve_const_param_in_enum_discriminant)]
    Const,
    #[note(resolve_lifetime_param_in_enum_discriminant)]
    Lifetime,
}

#[derive(Subdiagnostic)]
#[label(resolve_change_import_binding)]
pub(crate) struct ChangeImportBinding {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_change_import_binding,
    code = "{suggestion}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct ChangeImportBindingSuggestion {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) suggestion: String,
}

#[derive(Diagnostic)]
#[diag(resolve_imports_cannot_refer_to)]
pub(crate) struct ImportsCannotReferTo<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) what: &'a str,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_find_ident_in_this_scope)]
pub(crate) struct CannotFindIdentInThisScope<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) expected: &'a str,
    pub(crate) ident: Ident,
}

#[derive(Subdiagnostic)]
#[note(resolve_explicit_unsafe_traits)]
pub(crate) struct ExplicitUnsafeTraits {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Subdiagnostic)]
#[note(resolve_missing_macro_rules_name)]
pub(crate) struct MaybeMissingMacroRulesName {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[help(resolve_added_macro_use)]
pub(crate) struct AddedMacroUse;

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_consider_adding_a_derive,
    code = "{suggestion}",
    applicability = "maybe-incorrect"
)]
pub(crate) struct ConsiderAddingADerive {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) suggestion: String,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_determine_import_resolution)]
pub(crate) struct CannotDetermineImportResolution {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_determine_macro_resolution)]
#[note]
pub(crate) struct CannotDetermineMacroResolution {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) kind: &'static str,
    pub(crate) path: String,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_private, code = E0364)]
pub(crate) struct CannotBeReexportedPrivate {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_crate_public, code = E0364)]
pub(crate) struct CannotBeReexportedCratePublic {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_private, code = E0365)]
#[note(resolve_consider_declaring_with_pub)]
pub(crate) struct CannotBeReexportedPrivateNS {
    #[primary_span]
    #[label(resolve_reexport_of_private)]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_be_reexported_crate_public, code = E0365)]
#[note(resolve_consider_declaring_with_pub)]
pub(crate) struct CannotBeReexportedCratePublicNS {
    #[primary_span]
    #[label(resolve_reexport_of_crate_public)]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Subdiagnostic)]
#[help(resolve_consider_adding_macro_export)]
pub(crate) struct ConsiderAddingMacroExport {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[note(resolve_consider_marking_as_pub)]
pub(crate) struct ConsiderMarkingAsPub {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_glob_import_possible_crates)]
pub(crate) struct CannotGlobImportAllCrates {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_items_in_traits_are_not_importable)]
pub(crate) struct ItemsInTraitsAreNotImportable {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_is_not_directly_importable, code = E0253)]
pub(crate) struct IsNotDirectlyImportable {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) target: Ident,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unexpected_res_change_ty_to_const_param_sugg,
    code = "const ",
    style = "verbose"
)]
pub(crate) struct UnexpectedResChangeTyToConstParamSugg {
    #[primary_span]
    pub span: Span,
    #[applicability]
    pub applicability: Applicability,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_unexpected_res_use_at_op_in_slice_pat_with_range_sugg,
    code = "{snippet}",
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub(crate) struct UnexpectedResUseAtOpInSlicePatWithRangeSugg {
    #[primary_span]
    pub span: Span,
    pub ident: Ident,
    pub snippet: String,
}

#[derive(Diagnostic)]
#[diag(resolve_extern_crate_loading_macro_not_at_crate_root, code = E0468)]
pub(crate) struct ExternCrateLoadingMacroNotAtCrateRoot {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_bad_macro_import, code = E0466)]
pub(crate) struct BadMacroImport {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_extern_crate_self_requires_renaming)]
pub(crate) struct ExternCrateSelfRequiresRenaming {
    #[primary_span]
    #[suggestion(code = "extern crate self as name;", applicability = "has-placeholders")]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_macro_use_name_already_in_use)]
pub(crate) struct MacroUseNameAlreadyInUse {
    #[primary_span]
    #[note]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}

#[derive(Diagnostic)]
#[diag(resolve_imported_macro_not_found, code = E0469)]
pub(crate) struct ImportedMacroNotFound {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_macro_extern_deprecated)]
pub(crate) struct MacroExternDeprecated {
    #[primary_span]
    pub(crate) span: Span,
    #[note]
    pub inner_attribute: Option<()>,
}

#[derive(Diagnostic)]
#[diag(resolve_arguments_macro_use_not_allowed)]
pub(crate) struct ArgumentsMacroUseNotAllowed {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_unnamed_crate_root_import)]
pub(crate) struct UnnamedCrateRootImport {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_macro_expanded_extern_crate_cannot_shadow_extern_arguments)]
pub(crate) struct MacroExpandedExternCrateCannotShadowExternArguments {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_elided_anonymous_lifetime_report_error, code = E0637)]
pub(crate) struct ElidedAnonymousLivetimeReportError {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    #[subdiagnostic]
    pub(crate) suggestion: Option<ElidedAnonymousLivetimeReportErrorSuggestion>,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_elided_anonymous_lifetime_report_error_suggestion,
    applicability = "machine-applicable"
)]
pub(crate) struct ElidedAnonymousLivetimeReportErrorSuggestion {
    #[suggestion_part(code = "for<'a> ")]
    pub(crate) lo: Span,
    #[suggestion_part(code = "'a ")]
    pub(crate) hi: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_explicit_anonymous_lifetime_report_error, code = E0637)]
pub(crate) struct ExplicitAnonymousLivetimeReportError {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_implicit_elided_lifetimes_not_allowed_here, code = E0726)]
pub(crate) struct ImplicitElidedLifetimeNotAllowedHere {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_underscore_lifetime_is_reserved, code = E0637)]
pub(crate) struct UnderscoreLifetimeIsReserved {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_static_lifetime_is_reserved, code = E0262)]
pub(crate) struct StaticLifetimeIsReserved {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) lifetime: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_attempt_to_define_builtin_macro_twice, code = E0773)]
pub(crate) struct AttemptToDefineBuiltinMacroTwice {
    #[primary_span]
    pub(crate) span: Span,
    #[note]
    pub(crate) note_span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_variable_is_not_bound_in_all_patterns, code = E0408)]
pub(crate) struct VariableIsNotBoundInAllPatterns {
    #[primary_span]
    pub(crate) multispan: MultiSpan,
    pub(crate) name: Symbol,
}

#[derive(Subdiagnostic, Debug, Clone)]
#[label(resolve_pattern_doesnt_bind_name)]
pub(crate) struct PatternDoesntBindName {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) name: Symbol,
}

#[derive(Subdiagnostic, Debug, Clone)]
#[label(resolve_variable_not_in_all_patterns)]
pub(crate) struct VariableNotInAllPatterns {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
pub(crate) enum FailedToResolve {
    #[diag(resolve_failed_resolve_unresolve_import, code = E0433)]
    UnresolveImport {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_maybe_missing_crate, code = E0433)]
    MaybeMissingCrate {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_could_not_find_ident_in_root, code = E0433)]
    CouldNotFindIdentInRoot {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_could_not_find_ident_in_imported_crates, code = E0433)]
    CouldNotFindIdentInImportedCrates {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_could_not_find_ident_in_parent, code = E0433)]
    CouldNotFindIdentInParent {
        #[primary_span]
        span: Span,
        ident: Ident,
        parent: Symbol,
    },
    #[diag(resolve_expected_found, code = E0433)]
    ExpectedFound {
        #[primary_span]
        span: Span,
        ns_descr: &'static str,
        what: &'static str,
        ident: Ident,
        parent: Symbol,
    },
    #[diag(resolve_self_cannot_be_used_in_imports, code = E0433)]
    SelfCannotBeUsedInImports {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_self_is_only_available_in, code = E0433)]
    SelfImportsOnlyAllowedIn {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_undeclared_type, code = E0433)]
    UndeclaredType {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_is_not_crate_or_module, code = E0433)]
    IsNotCrateOrModule {
        #[primary_span]
        span: Span,
        descr: &'static str,
        ident: Ident,
    },
    #[diag(resolve_undeclared_crate_or_module, code = E0433)]
    UndeclaredCrateOrModule {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_global_paths_cannot_start_with_root, code = E0433)]
    GlobalPathCannotStartWithRoot {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_global_paths_cannot_start_with_name, code = E0433)]
    GlobalPathCannotStartWithName {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
    #[diag(resolve_root_only_in_start_position, code = E0433)]
    RootOnlyInStartPosition {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_name_only_in_start_position, code = E0433)]
    NameOnlyInStartPosition {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
    #[diag(resolve_too_many_super_keyword, code = E0433)]
    TooManySuperKeyword {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_ident_is_not_module, code = E0433)]
    IdentIsNotModule {
        #[primary_span]
        span: Span,
        ident: Ident,
        article: &'static str,
        descr: &'static str,
    },
    #[diag(resolve_partially_resolved_path, code = E0433)]
    PartiallyResolvedPath {
        #[primary_span]
        span: Span,
        article: &'static str,
        descr: &'static str,
    },
    #[diag(resolve_cannot_glob_import_into_itself, code = E0433)]
    CannotGlobImportIntoItself {
        #[primary_span]
        span: Span,
    },
    #[diag(resolve_no_ident_in_module, code = E0433)]
    NoIdentInModule {
        #[primary_span]
        span: Span,
        ident: Ident,
        module: String,
    },
    #[diag(resolve_no_ident_in_root, code = E0433)]
    NoIdentInRoot {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_no_external_crate_ident, code = E0433)]
    NoExternalCrateIdent {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[diag(resolve_failed_to_resolve, code = E0433)]
    EmptyLabel,
}

#[derive(Diagnostic)]
#[diag(resolve_unresolve_import, code = E0432)]
pub(crate) struct UnresolvedImport {
    #[primary_span]
    pub(crate) span: MultiSpan,
    pub(crate) number: usize,
    pub(crate) paths: String,
}

/// the label should be the same as the one of FailedToResolve
#[derive(Subdiagnostic, Debug, Clone)]
pub(crate) enum FailedToResolveLabel {
    #[label(resolve_failed_resolve_unresolve_import_label)]
    UnresolveImport {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_maybe_missing_crate_label)]
    MaybeMissingCrate {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_could_not_find_ident_in_root_label)]
    CouldNotFindIdentInRoot {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_could_not_find_ident_in_imported_crates_label)]
    CouldNotFindIdentInImportedCrates {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_could_not_find_ident_in_parent_label)]
    CouldNotFindIdentInParent {
        #[primary_span]
        span: Span,
        ident: Ident,
        parent: Symbol,
    },
    #[label(resolve_expected_found_label)]
    ExpectedFound {
        #[primary_span]
        span: Span,
        ns_descr: &'static str,
        what: &'static str,
        ident: Ident,
        parent: Symbol,
    },
    #[label(resolve_self_cannot_be_used_in_imports_label)]
    SelfCannotBeUsedInImports {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_self_is_only_available_in_label)]
    SelfImportsOnlyAllowedIn {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_undeclared_type_label)]
    UndeclaredType {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_is_not_crate_or_module_label)]
    IsNotCrateOrModule {
        #[primary_span]
        span: Span,
        descr: &'static str,
        ident: Ident,
    },
    #[label(resolve_undeclared_crate_or_module_label)]
    UndeclaredCrateOrModule {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_global_paths_cannot_start_with_root_label)]
    GlobalPathCannotStartWithRoot {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_global_paths_cannot_start_with_name_label)]
    GlobalPathCannotStartWithName {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
    #[label(resolve_root_only_in_start_position_label)]
    RootOnlyInStartPosition {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_name_only_in_start_position_label)]
    NameOnlyInStartPosition {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
    #[label(resolve_too_many_super_keyword_label)]
    TooManySuperKeyword {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_ident_is_not_module_label)]
    IdentIsNotModule {
        #[primary_span]
        span: Span,
        ident: Ident,
        article: &'static str,
        descr: &'static str,
    },
    #[label(resolve_partially_resolved_path_label)]
    PartiallyResolvedPath {
        #[primary_span]
        span: Span,
        article: &'static str,
        descr: &'static str,
    },
    #[label(resolve_cannot_glob_import_into_itself_label)]
    CannotGlobImportIntoItself {
        #[primary_span]
        span: Span,
    },
    #[label(resolve_no_ident_in_module_label)]
    NoIdentInModule {
        #[primary_span]
        span: Span,
        ident: Ident,
        module: String,
    },
    #[label(resolve_no_ident_in_root_label)]
    NoIdentInRoot {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    #[label(resolve_no_external_crate_ident_label)]
    NoExternalCrateIdent {
        #[primary_span]
        span: Span,
        ident: Ident,
    },
    EmptyLabel,
}

impl FailedToResolveLabel {
    pub(crate) fn set_span(&mut self, new_span: Span) {
        match self {
            Self::UnresolveImport { span }
            | Self::MaybeMissingCrate { span, .. }
            | Self::CouldNotFindIdentInRoot { span, .. }
            | Self::CouldNotFindIdentInImportedCrates { span, .. }
            | Self::CouldNotFindIdentInParent { span, .. }
            | Self::ExpectedFound { span, .. }
            | Self::SelfCannotBeUsedInImports { span }
            | Self::SelfImportsOnlyAllowedIn { span }
            | Self::UndeclaredType { span, .. }
            | Self::IsNotCrateOrModule { span, .. }
            | Self::UndeclaredCrateOrModule { span, .. }
            | Self::GlobalPathCannotStartWithRoot { span }
            | Self::GlobalPathCannotStartWithName { span, .. }
            | Self::RootOnlyInStartPosition { span }
            | Self::NameOnlyInStartPosition { span, .. }
            | Self::TooManySuperKeyword { span }
            | Self::IdentIsNotModule { span, .. }
            | Self::PartiallyResolvedPath { span, .. }
            | Self::CannotGlobImportIntoItself { span }
            | Self::NoIdentInModule { span, .. }
            | Self::NoIdentInRoot { span, .. }
            | Self::NoExternalCrateIdent { span, .. } => *span = new_span,
            Self::EmptyLabel => (),
        }
    }

    pub(crate) fn into_error(self) -> FailedToResolve {
        match self {
            FailedToResolveLabel::UnresolveImport { span } => {
                FailedToResolve::UnresolveImport { span }
            }
            FailedToResolveLabel::MaybeMissingCrate { span, ident } => {
                FailedToResolve::MaybeMissingCrate { span, ident }
            }
            FailedToResolveLabel::CouldNotFindIdentInRoot { span, ident } => {
                FailedToResolve::CouldNotFindIdentInRoot { span, ident }
            }
            FailedToResolveLabel::CouldNotFindIdentInImportedCrates { span, ident } => {
                FailedToResolve::CouldNotFindIdentInImportedCrates { span, ident }
            }
            FailedToResolveLabel::CouldNotFindIdentInParent { span, ident, parent } => {
                FailedToResolve::CouldNotFindIdentInParent { span, ident, parent }
            }
            FailedToResolveLabel::ExpectedFound { span, ns_descr, what, ident, parent } => {
                FailedToResolve::ExpectedFound { span, ns_descr, what, ident, parent }
            }
            FailedToResolveLabel::SelfCannotBeUsedInImports { span } => {
                FailedToResolve::SelfCannotBeUsedInImports { span }
            }
            FailedToResolveLabel::SelfImportsOnlyAllowedIn { span } => {
                FailedToResolve::SelfImportsOnlyAllowedIn { span }
            }
            FailedToResolveLabel::UndeclaredType { span, ident } => {
                FailedToResolve::UndeclaredType { span, ident }
            }
            FailedToResolveLabel::IsNotCrateOrModule { span, descr, ident } => {
                FailedToResolve::IsNotCrateOrModule { span, descr, ident }
            }
            FailedToResolveLabel::UndeclaredCrateOrModule { span, ident } => {
                FailedToResolve::UndeclaredCrateOrModule { span, ident }
            }
            FailedToResolveLabel::GlobalPathCannotStartWithRoot { span } => {
                FailedToResolve::GlobalPathCannotStartWithRoot { span }
            }
            FailedToResolveLabel::GlobalPathCannotStartWithName { span, name } => {
                FailedToResolve::GlobalPathCannotStartWithName { span, name }
            }
            FailedToResolveLabel::RootOnlyInStartPosition { span } => {
                FailedToResolve::RootOnlyInStartPosition { span }
            }
            FailedToResolveLabel::NameOnlyInStartPosition { span, name } => {
                FailedToResolve::NameOnlyInStartPosition { span, name }
            }
            FailedToResolveLabel::TooManySuperKeyword { span } => {
                FailedToResolve::TooManySuperKeyword { span }
            }
            FailedToResolveLabel::IdentIsNotModule { span, ident, article, descr } => {
                FailedToResolve::IdentIsNotModule { span, ident, article, descr }
            }
            FailedToResolveLabel::PartiallyResolvedPath { span, article, descr } => {
                FailedToResolve::PartiallyResolvedPath { span, article, descr }
            }
            FailedToResolveLabel::CannotGlobImportIntoItself { span } => {
                FailedToResolve::CannotGlobImportIntoItself { span }
            }
            FailedToResolveLabel::NoIdentInModule { span, ident, module } => {
                FailedToResolve::NoIdentInModule { span, ident, module }
            }
            FailedToResolveLabel::NoIdentInRoot { span, ident } => {
                FailedToResolve::NoIdentInRoot { span, ident }
            }
            FailedToResolveLabel::NoExternalCrateIdent { span, ident } => {
                FailedToResolve::NoExternalCrateIdent { span, ident }
            }
            FailedToResolveLabel::EmptyLabel => FailedToResolve::EmptyLabel,
        }
    }
}

#[derive(Subdiagnostic, Debug, Clone)]
pub(crate) enum FailedToResolveHelpOrSuggestion {
    #[multipart_suggestion(
        resolve_failed_to_resolve_suggest_remove_exclamation_mark,
        applicability = "maybe-incorrect"
    )]
    SuggestRemoveExclamationMark {
        #[suggestion_part(code = "")]
        exclamation_span: Span,
        path: String,
        kind: &'static str,
    },
    #[multipart_suggestion(resolve_similar_path_exists, applicability = "maybe-incorrect")]
    SimilarPathExists {
        #[suggestion_part(code = "{path}")]
        span: Span,
        path: String,
    },
    #[multipart_suggestion(resolve_using_std_instead_of_core, applicability = "maybe-incorrect")]
    TryUsingStdInsteadOfCore {
        #[suggestion_part(code = "std")]
        span: Span,
    },
    #[help(resolve_consider_adding_extern_crate)]
    ConsiderAddingExternCrate,
    #[multipart_suggestion(resolve_ident_is_not_a_type, applicability = "maybe-incorrect")]
    IdentIsNotAType {
        #[suggestion_part(code = "")]
        span: Span,
    },
    #[help(resolve_add_extern_crate_alloc)]
    AddExternCrateAlloc,
    #[multipart_suggestion(
        resolve_crate_module_with_similar_name,
        applicability = "maybe-incorrect"
    )]
    CrateModuleWithSimilarName {
        #[suggestion_part(code = "{suggestion}")]
        span: Span,
        suggestion: Symbol,
    },
    #[multipart_suggestion(resolve_similar_name_in_module, applicability = "maybe-incorrect")]
    SimilarNameInModule {
        #[suggestion_part(code = "{suggestion}")]
        span: Span,
        suggestion: Symbol,
    },
    /// see `Resolver::check_for_module_export_macro` in diagnostics for details
    #[multipart_suggestion(resolve_macro_with_this_name_at_crate_root)]
    MacroWithThisNameAtCrateRoot {
        #[suggestion_part(code = "{module_name}::{import_snippet}")]
        import: Option<Span>,
        module_name: Symbol,
        import_snippet: String,
        #[suggestion_part(code = "")]
        removal: Option<Span>,
        #[suggestion_part(code = "{start_snippet}{import_snippet}, ")]
        start_point_nested: Option<Span>,
        #[suggestion_part(code = "{{{import_snippet}, {start_snippet}")]
        start_point_not_nested: Option<Span>,
        start_snippet: String,
        #[suggestion_part(code = "}};")]
        end_point_not_nested: Option<Span>,
        #[suggestion_part(code = "use {module_name}::{import_snippet};\n")]
        module_relative_root_import: Option<Span>,
    },
}

#[derive(Subdiagnostic, Debug, Clone)]
pub(crate) enum FailedToResolveNote {
    #[note(resolve_macro_exported_at_crate_root)]
    MacroExportedAtCrateRoot,
    #[note(resolve_use_statements_changed_rust_2018)]
    UseStatementsChangedRust2018,
}

#[derive(Diagnostic)]
#[diag(resolve_name_defined_multiple_time)]
#[note]
pub(crate) struct NameDefinedMultipleTime {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) descr: &'static str,
    pub(crate) container: &'static str,
    #[subdiagnostic]
    pub(crate) label: NameDefinedMultipleTimeLabel,
    #[subdiagnostic]
    pub(crate) old_binding_label: Option<NameDefinedMultipleTimeOldBindingLabel>,
}

#[derive(Subdiagnostic)]
pub(crate) enum NameDefinedMultipleTimeLabel {
    #[label(resolve_name_defined_multiple_time_reimported)]
    Reimported {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
    #[label(resolve_name_defined_multiple_time_redefined)]
    Redefined {
        #[primary_span]
        span: Span,
        name: Symbol,
    },
}

#[derive(Subdiagnostic)]
pub(crate) enum NameDefinedMultipleTimeOldBindingLabel {
    #[label(resolve_name_defined_multiple_time_old_binding_import)]
    Import {
        #[primary_span]
        span: Span,
        name: Symbol,
        old_kind: &'static str,
    },
    #[label(resolve_name_defined_multiple_time_old_binding_definition)]
    Definition {
        #[primary_span]
        span: Span,
        name: Symbol,
        old_kind: &'static str,
    },
}

#[derive(Diagnostic)]
#[diag(resolve_is_private, code = E0603)]
pub(crate) struct IsPrivate<'a> {
    #[primary_span]
    #[label]
    pub(crate) span: Span,
    pub(crate) ident_descr: &'a str,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_generic_arguments_in_macro_path)]
pub(crate) struct GenericArgumentsInMacroPath {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_attributes_starting_with_rustc_are_reserved)]
pub(crate) struct AttributesStartingWithRustcAreReserved {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_use_through_an_import)]
pub(crate) struct CannotUseThroughAnImport {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) article: &'static str,
    pub(crate) descr: &'static str,
    #[note]
    pub(crate) binding_span: Option<Span>,
}

#[derive(Diagnostic)]
#[diag(resolve_name_reserved_in_attribute_namespace)]
pub(crate) struct NameReservedInAttributeNamespace {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_cannot_find_builtin_macro_with_name)]
pub(crate) struct CannotFindBuiltinMacroWithName {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
}

#[derive(Diagnostic)]
#[diag(resolve_tool_was_already_registered)]
pub(crate) struct ToolWasAlreadyRegistered {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) tool: Ident,
    #[label]
    pub(crate) old_ident_span: Span,
}

#[derive(Diagnostic)]
#[diag(resolve_tool_only_accepts_identifiers)]
pub(crate) struct ToolOnlyAcceptsIdentifiers {
    #[label]
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) tool: Symbol,
}

#[derive(Subdiagnostic)]
pub(crate) enum DefinedHere {
    #[label(resolve_similarly_named_defined_here)]
    SimilarlyNamed {
        #[primary_span]
        span: Span,
        descr: &'static str,
        candidate: Symbol,
    },
    #[label(resolve_single_item_defined_here)]
    SingleItem {
        #[primary_span]
        span: Span,
        descr: &'static str,
        candidate: Symbol,
    },
}

#[derive(Subdiagnostic)]
#[label(resolve_outer_ident_is_not_publicly_reexported)]
pub(crate) struct OuterIdentIsNotPubliclyReexported {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) descr: &'static str,
    pub(crate) outer_ident: Ident,
}

#[derive(Subdiagnostic)]
#[label(resolve_constructor_private_if_any_field_private)]
pub(crate) struct ConstructorPrivateIfAnyFieldPrivate {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    resolve_consider_making_the_field_public,
    applicability = "maybe-incorrect",
    style = "verbose"
)]
pub(crate) struct ConsiderMakingTheFieldPublic {
    #[suggestion_part(code = "pub ")]
    pub(crate) spans: Vec<Span>,
    pub(crate) number: usize,
}

#[derive(Subdiagnostic)]
pub(crate) enum ImportIdent {
    #[suggestion(
        resolve_suggestion_import_ident_through_reexport,
        code = "{path}",
        applicability = "machine-applicable",
        style = "verbose"
    )]
    ThroughReExport {
        #[primary_span]
        span: Span,
        ident: Ident,
        path: String,
    },
    #[suggestion(
        resolve_suggestion_import_ident_directly,
        code = "{path}",
        applicability = "machine-applicable",
        style = "verbose"
    )]
    Directly {
        #[primary_span]
        span: Span,
        ident: Ident,
        path: String,
    },
}

#[derive(Subdiagnostic)]
#[note(resolve_note_and_refers_to_the_item_defined_here)]
pub(crate) struct NoteAndRefersToTheItemDefinedHere<'a> {
    #[primary_span]
    pub(crate) span: MultiSpan,
    pub(crate) item: &'a str,
    pub(crate) name: Ident,
    pub(crate) first: bool,
    pub(crate) dots: bool,
}

#[derive(Subdiagnostic)]
#[suggestion(
    resolve_remove_unnecessary_import,
    code = "",
    applicability = "maybe-incorrect",
    style = "tool-only"
)]
pub(crate) struct RemoveUnnecessaryImport {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[note(resolve_ident_imported_here_but_it_is_desc)]
pub(crate) struct IdentImporterHereButItIsDesc<'a> {
    #[primary_span]
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) desc: &'a str,
}

#[derive(Subdiagnostic)]
#[note(resolve_ident_in_scope_but_it_is_desc)]
pub(crate) struct IdentInScopeButItIsDesc<'a> {
    pub(crate) ident: Ident,
    pub(crate) desc: &'a str,
}

#[derive(Subdiagnostic)]
#[note(resolve_found_an_item_configured_out)]
pub(crate) struct FoundItemConfigureOut {
    #[primary_span]
    pub(crate) span: Span,
}

#[derive(Subdiagnostic)]
#[note(resolve_item_was_behind_feature)]
pub(crate) struct ItemWasBehindFeature {
    pub(crate) feature: Symbol,
}
