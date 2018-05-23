#!/usr/bin/env python2

import re
import pprint
import subprocess

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
    print "|%21s|%s|" % (feature, result_details)

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
results['simulacrum']['rustc'] = "stable ok"

results['double']['first_release'] = "Dec-12-2017 -"
results['galvanic_mock']['first_release'] = "Aug-13-2017 -"
results['mock_derive']['first_release'] = "Jul-16-2017 -"
results['mockers']['first_release'] = "Apr-6-2016 -"
results['simulacrum']['first_release'] = "Dec-17-2017 -"

# Finally, generate the table
libnames = sorted(results.keys())
lib_headers = "|_. ".join(libnames)
print "|_. |_.%s|" % lib_headers
essential_features = ["associated_types", "checkpoint", "consume",
"doctest", "external_trait", "fallback", "foreign", "generic_parameters",
"generic_return", "generic_trait", "inherited_trait", "match_method",
"mock_struct", "multi_trait", "return_call_with_args", "return_lifetime",
"return_owned", "return_parameters", "sequence", "static_method",
"times_range",]
convenience_features = [ "derive", "match_and", "match_constant",
"match_operator", "match_or", "match_pattern", "match_range", "match_wildcard",
"return_call", "return_constant", "return_default", "return_panic",
"times_once", "times_any", "times_n", "times_never"]
other_features = [ "many_args", "rustc", "first_release"]
print "|\\6=. Essential Features|"
for feature in essential_features:
    print_row(feature, results)
print "|\\6=. Convenience Features|"
for feature in convenience_features:
    print_row(feature, results)
print "|\\6=. Other|"
for feature in other_features:
    print_row(feature, results)
