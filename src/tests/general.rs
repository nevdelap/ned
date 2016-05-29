/// Just a few general tests. The specifics are tested in the other test files.

use ned;

#[test]
fn basic_match() {

    let args = "accidentally test --whole-files";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["test/file1.txt:\nThe accidentally ghastly hand plans an escape from a cream puff the \
          placid widow. A slovenly\nonlooker rejoices, because some single-handledly sheepish \
          stalactite knowingly avoids contact with a\nwisely rhetorical ballerina. Sometimes the \
          waif about a swamp rejoices, but a ruffian always barely\nbefriends an unseemly \
          dilettante! Unlike so many mastadons who have made their lovely widow\nabhorrent to \
          us, waifs remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn basic_match_file_names_only() {

    let args = "accidentally test --whole-files --filenames-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/file1.txt\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn basic_match_no_file_names() {

    let args = "accidentally test --whole-files --no-filenames";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["The accidentally ghastly hand plans an escape from a cream puff the placid widow. A \
          slovenly\nonlooker rejoices, because some single-handledly sheepish stalactite \
          knowingly avoids contact with a\nwisely rhetorical ballerina. Sometimes the waif about \
          a swamp rejoices, but a ruffian always barely\nbefriends an unseemly dilettante! \
          Unlike so many mastadons who have made their lovely widow\nabhorrent to us, waifs \
          remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn basic_match_file_names_only_no_match() {

    let args = "secretly test/dir1 --whole-files --filenames-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/dir1/file2.txt\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn basic_match_no_file_names_no_match() {

    let args = "secretly test/dir1 --whole-files --no-filenames --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = [""];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn only_matches() {

    let args = "accidentally.*hand test --whole-files --matches-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/file1.txt:\naccidentally ghastly hand\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match() {

    let args = "accidentally.*hand test --whole-files --colors";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\u{1b}[35mtest/file1.txt\u{1b}[0m:\nThe \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m \
          plans an escape from a cream puff the placid widow. A slovenly\nonlooker rejoices, \
          because some single-handledly sheepish stalactite knowingly avoids contact with \
          a\nwisely rhetorical ballerina. Sometimes the waif about a swamp rejoices, but a \
          ruffian always barely\nbefriends an unseemly dilettante! Unlike so many mastadons who \
          have made their lovely widow\nabhorrent to us, waifs remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_file_names_only() {

    let args = "accidentally.*hand test --whole-files --colors --filenames-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35mtest/file1.txt\u{1b}[0m\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_no_file_names() {

    let args = "accidentally.*hand test --whole-files --colors --no-filenames";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["The \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m plans an escape from a cream puff \
          the placid widow. A slovenly\nonlooker rejoices, because some single-handledly \
          sheepish stalactite knowingly avoids contact with a\nwisely rhetorical ballerina. \
          Sometimes the waif about a swamp rejoices, but a ruffian always barely\nbefriends an \
          unseemly dilettante! Unlike so many mastadons who have made their lovely \
          widow\nabhorrent to us, waifs remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_file_names_only_no_match() {

    let args = "secretly test/dir1 --whole-files --colors --filenames-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35mtest/dir1/file2.txt\u{1b}[0m\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn duplicate_options() {

    let args = "accidentally.*hand test --whole-files --colors --colors -c";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\u{1b}[35mtest/file1.txt\u{1b}[0m:\nThe \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m \
          plans an escape from a cream puff the placid widow. A slovenly\nonlooker rejoices, \
          because some single-handledly sheepish stalactite knowingly avoids contact with \
          a\nwisely rhetorical ballerina. Sometimes the waif about a swamp rejoices, but a \
          ruffian always barely\nbefriends an unseemly dilettante! Unlike so many mastadons who \
          have made their lovely widow\nabhorrent to us, waifs remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn no_context_match() {

    let args = "is test --include long*.txt";
    let expected_exit_code = 0;
    let expected_screen_output = ["\
test/longfile.txt: The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt: Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt: sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt: looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt: The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt: caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt: gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt: soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt: from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt: where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt: cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt: fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt: her the lovely fetishist with a cup beyond the pocket, and lazily
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

// #[test] TODO NEXT: enable and do TODOs in main.rs.
fn context_after_1_match() {

    let args = "is test --include long*.txt";
    let expected_exit_code = 0;
    let expected_screen_output = ["\
test/longfile.txt: The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt: Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt: sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt: looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt: where we can secretly give lectures on morality to our cream puff.
test/longfile.txt: The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt: glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt: caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt: gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt: need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt: soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt: from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt: a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt: where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt: shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt: cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt: lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt: fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt: teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt: her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt: boogies the dark side of her
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match() {

    let args = "her test --whole-files --recursive";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["test/dir1/dir4/file6.txt:\nThe shadow conquers the hand related to a mastadon. Jespera \
          and I took a cup around a toothache\n(with a lunatic around some debutante, a ribbon \
          beyond a curse, a few dahlias, and a ribbon) to\narrive at a state of intimacy where \
          we can accurately mourn our boy. When another espadrille wakes\nup, the cup toward \
          another swamp flies into a rage. Now and then, an onlooker sells a dissident\nrelated \
          to the hand to an ungodly dahlia.\n",
         "test/dir1/dir4/dir5/file7.txt:\nA bicep near the mastadon has a change of heart about \
          a lovely snow. A halfhearted curse steals\npencils from the maestro. He called her the \
          Interloper (or was it Timosha?). Now and then, the\ntrombone almost gives secret \
          financial aid to the wisely strawberry-blonde marzipan.\n",
         "test/dir1/file2.txt:\nThe omphalos toward a bubble bath is lowly. The unsightly bicep \
          panics, and a toothpick feels\nnagging remorse; however, a bubble toward a bonbon pees \
          on an almost ghastly lunatic. Desdemona,\nalthough somewhat soothed by a rapacious \
          trombone and the girl beyond a gypsy, still bestows great\nhonor upon her from another \
          onlooker beyond the maestro, bestow great honor upon her a tea party\nwith a bubble, \
          and sells a bonbon to the dark side of her looking glass. If the halfhearted \
          waif\nboogies a menag\u{e9} \u{e0} trois, then some bride beams with joy.\n",
         "test/dir3/file5.txt:\nAn onlooker, a curmudgeonly swamp, and a bubble are what got \
          Scheherazade into trouble. A slyly\nunruffled dissident is underhandedly curmudgeonly. \
          Harpo Marx and I took a coward (with a guardian\nangel living with a cream puff, the \
          bubble beyond a swamp, a few dahlias, and another starlet over a\nbonbon) to arrive at \
          a state of intimacy where we can wisely write a love letter to our taxidermist.\nThe \
          taxidermist goes to sleep, and some mirror flies into a rage; however, the guardian \
          angel\nrelated to some dilettante makes a truce with the lowly labyrinth.\n",
         "test/dir2/file4.txt:\nHarpo Marx and I took another hand for some espadrille (with an \
          irreconcilable tea party, a widow\nliving with the fetishist, a few midwifes, and a \
          clodhopper near some clock) to arrive at a state of\nintimacy where we can carelessly \
          slyly organize our necromancer. Another nefarious bubble teaches a\nseldom comely \
          necromancer. A coward gives a pink slip to a cleavage toward a bicep. The girl near \
          a\ntoothpick shares a shower with the ballerina.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match_file_names_only() {

    let args = "her test --whole-files --recursive --filenames-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/dir1/dir4/file6.txt\n",
                                  "test/dir1/dir4/dir5/file7.txt\n",
                                  "test/dir1/file2.txt\n",
                                  "test/dir3/file5.txt\n",
                                  "test/dir2/file4.txt\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match_no_filesnames() {

    let args = "her test --whole-files --recursive --no-filenames";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["The shadow conquers the hand related to a mastadon. Jespera and I took a cup around a \
          toothache\n(with a lunatic around some debutante, a ribbon beyond a curse, a few \
          dahlias, and a ribbon) to\narrive at a state of intimacy where we can accurately mourn \
          our boy. When another espadrille wakes\nup, the cup toward another swamp flies into a \
          rage. Now and then, an onlooker sells a dissident\nrelated to the hand to an ungodly \
          dahlia.\n",
         "A bicep near the mastadon has a change of heart about a lovely snow. A halfhearted \
          curse steals\npencils from the maestro. He called her the Interloper (or was it \
          Timosha?). Now and then, the\ntrombone almost gives secret financial aid to the wisely \
          strawberry-blonde marzipan.\n",
         "The omphalos toward a bubble bath is lowly. The unsightly bicep panics, and a \
          toothpick feels\nnagging remorse; however, a bubble toward a bonbon pees on an almost \
          ghastly lunatic. Desdemona,\nalthough somewhat soothed by a rapacious trombone and the \
          girl beyond a gypsy, still bestows great\nhonor upon her from another onlooker beyond \
          the maestro, bestow great honor upon her a tea party\nwith a bubble, and sells a \
          bonbon to the dark side of her looking glass. If the halfhearted waif\nboogies a \
          menag\u{e9} \u{e0} trois, then some bride beams with joy.\n",
         "An onlooker, a curmudgeonly swamp, and a bubble are what got Scheherazade into \
          trouble. A slyly\nunruffled dissident is underhandedly curmudgeonly. Harpo Marx and I \
          took a coward (with a guardian\nangel living with a cream puff, the bubble beyond a \
          swamp, a few dahlias, and another starlet over a\nbonbon) to arrive at a state of \
          intimacy where we can wisely write a love letter to our taxidermist.\nThe taxidermist \
          goes to sleep, and some mirror flies into a rage; however, the guardian angel\nrelated \
          to some dilettante makes a truce with the lowly labyrinth.\n",
         "Harpo Marx and I took another hand for some espadrille (with an irreconcilable tea \
          party, a widow\nliving with the fetishist, a few midwifes, and a clodhopper near some \
          clock) to arrive at a state of\nintimacy where we can carelessly slyly organize our \
          necromancer. Another nefarious bubble teaches a\nseldom comely necromancer. A coward \
          gives a pink slip to a cleavage toward a bicep. The girl near a\ntoothpick shares a \
          shower with the ballerina.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match_file_names_only_no_match() {

    let args = "her test --whole-files --recursive --filenames-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/dir1/file3.txt\n", "test/file1.txt\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

// These tests look for each of the file's matches it expects to be in the screen output, which
// can be in any order, because the order that walkdir walks directories is undefined.
fn test(args: &str, expected_exit_code: i32, expected_screen_output: &[&str]) {
    let args: Vec<String> = args.split_whitespace()
        .map(|arg| arg.to_string())
        .collect::<Vec<String>>();

    let mut screen_output: Vec<u8> = vec![];

    let exit_code = ned(&mut screen_output, &args).unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();

    assert_eq!(exit_code, expected_exit_code);
    for part in expected_screen_output.into_iter() {
        if !screen_output.contains(part) {
            println!("{:?} not in {:?}", part, screen_output);
            assert!(false);
        }
    }
}
