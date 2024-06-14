use std assert

export def main [] {
    let crates = ls -a crates | where type == dir 

    let not_updated = $crates | each {|el| 
        cd $el.name
        let readme = cargo readme
        let old_readme = open README.md --raw
        $readme + "\n" | save -f README.md 
        ($readme + "\n" == $old_readme)
    } | reduce {|a, b| $a and $b }
    assert $not_updated
}
