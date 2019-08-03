#!/usr/bin/env python2

import re
import pprint
import subprocess

descs = {
    "double": "Double",
    "galvanic_mock": "Galvanic-mock",
    "mock_derive": "Mock_Derive",
    "mock_it": "Mock-it",
    "mockall": "Mockall",
    "mockers": "Mockers",
    "mocktopus": "Mocktopus",
    "pseudo": "Pseudo",
    "simulacrum": "Simulacrum",

    "associated_types": "Associated types",
    "checkpoint": "Checkpoints",
    "reference_parameters": "Reference parameters",
    "consume_parameters": "Consume parameters",
    "consume_self": "Consume self",
    "doctest": "Doctest",
    "external_trait": "External traits",
    "foreign": "Foreign",
    "generic_method": "Generic methods",
    "generic_return": "Generic return",
    "generic_struct": "Generic structs",
    "generic_trait": "Generic traits",
    "inherited_trait": "Inherited traits",
    "match_method": "Match function",
    "mock_struct": "Structs",
    "mock_trait": "Traits",
    "multi_trait": "Multiple traits",
    "return_call_with_args": "Return call with args",
    "return_lifetime": "Return lifetime",
    "return_owned": "Return owned",
    "return_parameters": "Return parameters",
    "send": "Send",
    "sequence": "Sequence",
    "static_method": "Static methods",
    "times_range": "Times range",
    "where_clause": "Where clauses",

    "derive": "Derive",
    "fallback": "Fallback",
    "match_combo": "Match combinations",
    "match_constant": "Match constant",
    "match_operator": "Match operator",
    "match_pattern": "Match pattern",
    "match_range": "Match range",
    "match_wildcard": "Match wildcard",
    "modules": "Mock modules",
    "return_constant": "Return a constant",
    "return_default": "Return default",
    "return_panic": "Return panic",
    "times_once": "Times once",
    "times_any": "Times any",
    "times_n": "Times n",
    "times_never": "Times never",

    "many_args": "Maximum arguments",
    "rustc": "Rustc",
    "first_release": "First release",
    "version": "Tested version",
    "link": "Current version",
}

def format_cell(s):
    words = s.split(" ")
    result = words[-1]
    text = " ".join(words[0:-1])
    if '<img ' in text:
        bg = "white"
    elif result == "ok":
        if re.match("^0\.[0-9]+\.[0-9]+", text):
            bg = "#fe7d37"
        else:
            bg = "#ADEBAD"
    elif result == "warn":
        bg = "#FFEF99"
    elif result == "-":
        bg = "white"
    else:
        bg = "#EB9999"
    if not text:
        text = {"error": "no", "ok": "yes", "FAILED": "no"}[result]
    return "{background:%s}.%s" % (bg, text)

def print_row(feature, results):
    result_details = "|".join([format_cell(results[l][feature])
        for l in libnames])
    print "|%21s|%s|" % (descs[feature], result_details)

# First, run the tests and collect results
results = {}
p1 = subprocess.Popen(["cargo", "+nightly", "test", "-v",
    "--no-fail-fast", "--", "--nocapture", "--test-threads=1"],
    stdout=subprocess.PIPE)
output = p1.communicate()[0]
for line in output.splitlines():
    match = re.match("^test t_(\w+)::(?:mod_t::)?t::(\w+) \.\.\. (.+)$", line)
    if not match:
        match = re.match(
            "^test src/t_(\w+)\.rs - \w+::(doctest) \(line \d+\) \.\.\. (\w+)", line)
    if match:
        lib = match.group(1)
        feature = match.group(2)
        result = match.group(3)
        if not results.has_key(lib):
            results[lib] = {}
        results[lib][feature] = result

# Manually add a few more data
results['double']['rustc'] = "stable ok"
results['galvanic_mock']['rustc'] = "nightly warn"
# results['mock_derive']['rustc'] = "nightly < 1.28.0 error"
results['mockall']['rustc'] = "stable ok"
results['mockers']['rustc'] = "stable ok"
results['mocktopus']['rustc'] = "nightly warn"
results['pseudo']['rustc'] = "stable ok"
results['simulacrum']['rustc'] = "stable ok"
results['mock_it']['rustc'] = "stable ok"

results['double']['first_release'] = "Dec-12-2017 -"
results['galvanic_mock']['first_release'] = "Aug-13-2017 -"
# results['mock_derive']['first_release'] = "Jul-16-2017 -"
results['mockall']['first_release'] = "Jul-3-2019 -"
results['mockers']['first_release'] = "Apr-6-2016 -"
results['mocktopus']['first_release'] = "Sep-5-2017 -"
results['pseudo']['first_release'] = "Mar-23-2017 -"
results['simulacrum']['first_release'] = "Dec-17-2017 -"
results['mock_it']['first_release'] = "Mar-11-2018 -"

# Finally, generate the table
libnames = sorted(results.keys())
lib_headers = "|_. ".join([descs[l] for l in libnames])
print "|_. |_.%s|" % lib_headers
essential_features = ["associated_types", "checkpoint", "reference_parameters",
"consume_parameters", "consume_self", "doctest", "external_trait", "foreign",
"generic_method", "generic_return", "generic_struct", "generic_trait",
"inherited_trait", "match_method", "mock_struct", "mock_trait", "multi_trait",
"return_call_with_args", "return_lifetime", "return_owned", "return_parameters",
"send", "sequence", "static_method", "times_range", "where_clause"]
convenience_features = [ "derive", "fallback", "match_combo", "match_constant",
"match_operator", "match_pattern", "match_range", "match_wildcard", "modules",
"return_constant", "return_default", "return_panic",
"times_once", "times_any", "times_n", "times_never"]
other_features = [ "many_args", "rustc", "first_release", "version", "link"]
print "|\\8=. Essential Features|"
for feature in essential_features:
    print_row(feature, results)
print "|\\8=. Convenience Features|"
for feature in convenience_features:
    print_row(feature, results)
print "|\\8=. Other|"
for feature in other_features:
    print_row(feature, results)
