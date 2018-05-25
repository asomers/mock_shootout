#!/usr/bin/env python2

import re
import pprint
import subprocess

descs = {
    "double": "Double",
    "galvanic_mock": "Galvanic-mock",
    "mock_derive": "Mock_Derive",
    "mock_it": "Mock-it",
    "mockers": "Mockers",
    "pseudo": "Pseudo",
    "simulacrum": "Simulacrum",

    "associated_types": "Associated types",
    "checkpoint": "Checkpoints",
    "consume": "Consume",
    "doctest": "Doctest",
    "external_trait": "External traits",
    "fallback": "Fallback",
    "foreign": "Foreign",
    "generic_parameters": "Generic parameters",
    "generic_return": "Generic return",
    "generic_trait": "Generic traits",
    "inherited_trait": "Inherited traits",
    "match_method": "Match function",
    "mock_struct": "Structs",
    "multi_trait": "Multiple traits",
    "return_call_with_args": "Return call with args",
    "return_lifetime": "Return lifetime",
    "return_owned": "Return owned",
    "return_parameters": "Return parameters",
    "sequence": "Sequence",
    "static_method": "Static methods",
    "times_range": "Times range",

    "derive": "Derive",
    "match_combo": "Match combinations",
    "match_constant": "Match constant",
    "match_operator": "Match operator",
    "match_pattern": "Match pattern",
    "match_range": "Match range",
    "match_wildcard": "Match wildcard",
    "return_call": "Return call",
    "return_constant": "Return a constant",
    "return_default": "Return default",
    "return_panic": "Return panic",
    "times_once": "Times once",
    "times_any": "Times any",
    "times_n": "Times n",
    "times_never": "Times never",
    "many_args": "Maximum arguments",
    "rustc": "Rustc",
    "first_release": "First release"
}

def format_cell(s):
    words = s.split(" ")
    result = words[-1]
    text = " ".join(words[0:-1])
    if result == "ok":
        bg = "#ADEBAD"
    elif result == "warn":
        bg = "#FFEF99"
    elif result == "-":
        bg = "white"
    else:
        bg = "#EB9999"
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
results['mock_derive']['rustc'] = "nightly warn"
results['mockers']['rustc'] = "nightly warn"
results['pseudo']['rustc'] = "stable ok"
results['simulacrum']['rustc'] = "stable ok"
results['mock_it']['rustc'] = "stable ok"

results['double']['first_release'] = "Dec-12-2017 -"
results['galvanic_mock']['first_release'] = "Aug-13-2017 -"
results['mock_derive']['first_release'] = "Jul-16-2017 -"
results['mockers']['first_release'] = "Apr-6-2016 -"
results['pseudo']['first_release'] = "Mar-23-2017 -"
results['simulacrum']['first_release'] = "Dec-17-2017 -"
results['mock_it']['first_release'] = "Mar-11-2018 -"

# Finally, generate the table
libnames = sorted(results.keys())
lib_headers = "|_. ".join([descs[l] for l in libnames])
print "|_. |_.%s|" % lib_headers
essential_features = ["associated_types", "checkpoint", "consume",
"doctest", "external_trait", "fallback", "foreign", "generic_parameters",
"generic_return", "generic_trait", "inherited_trait", "match_method",
"mock_struct", "multi_trait", "return_call_with_args", "return_lifetime",
"return_owned", "return_parameters", "sequence", "static_method",
"times_range",]
convenience_features = [ "derive", "match_combo", "match_constant",
"match_operator", "match_pattern", "match_range", "match_wildcard",
"return_call", "return_constant", "return_default", "return_panic",
"times_once", "times_any", "times_n", "times_never"]
other_features = [ "many_args", "rustc", "first_release"]
print "|\\8=. Essential Features|"
for feature in essential_features:
    print_row(feature, results)
print "|\\8=. Convenience Features|"
for feature in convenience_features:
    print_row(feature, results)
print "|\\8=. Other|"
for feature in other_features:
    print_row(feature, results)
