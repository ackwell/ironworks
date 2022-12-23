// Characters to strip from schema struct keys
// TODO: this is potentially a bit saint-specific; but i'm very hesitant to put this logic in stc parsing, as that's technically "wrong". probably best shot is to keep this logic in tune with what BM requires as an output data format.
// TODO: The above is becoming less relevant at this point. While it still shouldn't be placed in StC logic in the schema package itself, it probably _should_ be run as a transformation over a schema before walking and suchforth is performed - both for single-schema reuse purposes, and also cacheability. i.e. on search, a single schema can be used for both query normalisation and result hydration.
const FIELD_STRIP_CHARACTERS: &[char] = &['{', '}', '[', ']', '<', '>'];

pub fn sanitize_name(name: &str) -> String {
	name.replace(FIELD_STRIP_CHARACTERS, "")
}
