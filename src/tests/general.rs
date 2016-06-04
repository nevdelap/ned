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
fn basic_match_line_numbers_only() {

    let args = "accidentally test --line-numbers-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["1\n"];

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
fn basic_match_no_line_numbers() {

    let args = "accidentally test --no-line-numbers";
    let expected_exit_code = 0;
    let expected_screen_output = ["test/file1.txt:The accidentally ghastly hand plans an escape \
                                   from a cream puff the placid widow. A slovenly"];

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
fn basic_match_line_numbers_only_no_match() {

    let args = "secretly test/dir1 --line-numbers-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["1\n2\n3\n4\n5\n6\n1\n2\n3\n"];

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
fn basic_match_no_line_numbers_no_match() {

    let args = "secretly test/dir1 --no-line-numbers --no-match";
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
        ["\u{1b}[35mtest/file1.txt:\n\u{1b}[0mThe \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m \
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
    let expected_screen_output = ["\u{1b}[35mtest/file1.txt\n\u{1b}[0m"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_line_numbers_only() {

    let args = "rejoices.*hand test --colors --line-numbers-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35m2\n\u{1b}[0m"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_no_file_names_whole_files() {

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
fn colored_match_no_file_names() {

    let args = "accidentally.*hand test --colors --no-filenames";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35m1:\u{1b}[0mThe \u{1b}[1;31maccidentally ghastly \
                                   hand\u{1b}[0m plans an escape from a cream puff the placid \
                                   widow. A slovenly\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_no_line_numbers() {

    let args = "accidentally.*hand test --colors --no-line-numbers";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35mtest/file1.txt:\u{1b}[0mThe \
                                   \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m plans an \
                                   escape from a cream puff the placid widow. A slovenly\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_file_names_only_no_match() {

    let args = "secretly test/dir1 --whole-files --colors --filenames-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35mtest/dir1/file2.txt\n\u{1b}[0m"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match_line_numbers_only_no_match() {

    let args = "secretly test/dir1 --colors --line-numbers-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["\u{1b}[35m1\n\u{1b}[0m\u{1b}[35m2\n\u{1b}[0m\u{1b}[35m3
\u{1b}[0m\u{1b}[35m4\n\u{1b}[0m\u{1b}[35m5\n\u{1b}[0m\u{1b}[35m6\n\u{1b}[0m", "\u{1b}[35m1
\u{1b}[0m\u{1b}[35m2\n\u{1b}[0m\u{1b}[35m3\n\u{1b}[0m"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn duplicate_options() {

    let args = "accidentally.*hand test --whole-files --colors --colors -c";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\u{1b}[35mtest/file1.txt:\n\u{1b}[0mThe \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m \
          plans an escape from a cream puff the placid widow. A slovenly\nonlooker rejoices, \
          because some single-handledly sheepish stalactite knowingly avoids contact with \
          a\nwisely rhetorical ballerina. Sometimes the waif about a swamp rejoices, but a \
          ruffian always barely\nbefriends an unseemly dilettante! Unlike so many mastadons who \
          have made their lovely widow\nabhorrent to us, waifs remain womanly.\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_0_match() {

    let args = "is test --include long*.txt";
    let expected_exit_code = 0;
    let expected_screen_output = ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_after_1_match() {

    let args = "is test --include long*.txt --after 1";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menag\u{e9} \u{e0} \
trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_before_1_match() {

    let args = "is test --include long*.txt --before 1";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_1_match() {

    let args = "is test --include long*.txt --context 1";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_after_2_match() {

    let args = "is test --include long*.txt --after 2";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:6:Toscanini, the friend of Jean-Pierre and Jacques, goes to sleep with
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:20:When a wobbly coward trembles, a ballerina for a cup rejoices. A
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:26:ballerina are what got Timosha into trouble. Sometimes the piroshki
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
test/longfile.txt:37:snow.\
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_before_2_match() {

    let args = "is test --include long*.txt --before 2";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:8:botched hand. A bubble inside a dilettante, a swamp for the lunatic,
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:22:(with a rhetorical haunch, another rascally cigar, a few omphaloss,
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:28:bestows great honor upon a wobbly clodhopper! A girl takes a peek at a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_2_match() {

    let args = "is test --include long*.txt --context 2";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:6:Toscanini, the friend of Jean-Pierre and Jacques, goes to sleep with
test/longfile.txt:8:botched hand. A bubble inside a dilettante, a swamp for the lunatic,
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:20:When a wobbly coward trembles, a ballerina for a cup rejoices. A
test/longfile.txt:22:(with a rhetorical haunch, another rascally cigar, a few omphaloss,
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:26:ballerina are what got Timosha into trouble. Sometimes the piroshki
test/longfile.txt:28:bestows great honor upon a wobbly clodhopper! A girl takes a peek at a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
test/longfile.txt:37:snow.
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_after_5_match() {

    let args = "is test --include long*.txt --after 5";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:6:Toscanini, the friend of Jean-Pierre and Jacques, goes to sleep with
test/longfile.txt:7:the wily labyrinth. The slovenly piroshki ostensibly teaches the
test/longfile.txt:8:botched hand. A bubble inside a dilettante, a swamp for the lunatic,
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:20:When a wobbly coward trembles, a ballerina for a cup rejoices. A
test/longfile.txt:21:gingerly curse conquers the philosopher. Toscanini and I took a bicep
test/longfile.txt:22:(with a rhetorical haunch, another rascally cigar, a few omphaloss,
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:26:ballerina are what got Timosha into trouble. Sometimes the piroshki
test/longfile.txt:27:toward a dahlia goes to sleep, but a girl about a piroshki always
test/longfile.txt:28:bestows great honor upon a wobbly clodhopper! A girl takes a peek at a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
test/longfile.txt:37:snow.
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_before_5_match() {

    let args = "is test --include long*.txt --before 5";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:6:Toscanini, the friend of Jean-Pierre and Jacques, goes to sleep with
test/longfile.txt:7:the wily labyrinth. The slovenly piroshki ostensibly teaches the
test/longfile.txt:8:botched hand. A bubble inside a dilettante, a swamp for the lunatic,
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:20:When a wobbly coward trembles, a ballerina for a cup rejoices. A
test/longfile.txt:21:gingerly curse conquers the philosopher. Toscanini and I took a bicep
test/longfile.txt:22:(with a rhetorical haunch, another rascally cigar, a few omphaloss,
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:26:ballerina are what got Timosha into trouble. Sometimes the piroshki
test/longfile.txt:27:toward a dahlia goes to sleep, but a girl about a piroshki always
test/longfile.txt:28:bestows great honor upon a wobbly clodhopper! A girl takes a peek at a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn context_5_match() {

    let args = "is test --include long*.txt --context 5";
    let expected_exit_code = 0;
    let expected_screen_output =
        ["\
test/longfile.txt:1:The bodice ripper writes a love letter to a comely dissident. The
test/longfile.txt:2:Interloper and I took a self-actualized dissident (with the wisely
test/longfile.txt:3:sprightly necromancer, another dissident, a few mirrors, and the
test/longfile.txt:4:looking glass near a taxidermist) to arrive at a state of intimacy
test/longfile.txt:5:where we can secretly give lectures on morality to our cream puff.
test/longfile.txt:6:Toscanini, the friend of Jean-Pierre and Jacques, goes to sleep with
test/longfile.txt:7:the wily labyrinth. The slovenly piroshki ostensibly teaches the
test/longfile.txt:8:botched hand. A bubble inside a dilettante, a swamp for the lunatic,
test/longfile.txt:9:and a trombone defined by a shadow are what got Nimbo into trouble.
test/longfile.txt:10:The dilettante defined by a clock ceases to exist, and the looking
test/longfile.txt:11:glass seeks the lovely trombone. The toothache hardly trades baseball
test/longfile.txt:12:cards with a amour-propre, but a non-chalantly sublime bubble almost
test/longfile.txt:13:caricatures a dissident. Most people believe that a ghastly gonad
test/longfile.txt:14:gives lectures on morality to the wisely darling toothpick, but they
test/longfile.txt:15:need to remember how hesitantly a bonbon daydreams. A widow somewhat
test/longfile.txt:16:caricatures the widow from a tea party. Nicolas, although somewhat
test/longfile.txt:17:soothed by an espadrille and a fetishist, still makes a truce with her
test/longfile.txt:18:from an unseemly gypsy, buy an expensive gift for her a fetishist with
test/longfile.txt:19:a philosopher, and takes a peek at the dark side of her dilettante.
test/longfile.txt:20:When a wobbly coward trembles, a ballerina for a cup rejoices. A
test/longfile.txt:21:gingerly curse conquers the philosopher. Toscanini and I took a bicep
test/longfile.txt:22:(with a rhetorical haunch, another rascally cigar, a few omphaloss,
test/longfile.txt:23:and the cleavage behind the bride) to arrive at a state of intimacy
test/longfile.txt:24:where we can feverishly play pinochle with our trombone. The boy for a
test/longfile.txt:25:shadow, a gypsy living with a boy, and some toothpick for another
test/longfile.txt:26:ballerina are what got Timosha into trouble. Sometimes the piroshki
test/longfile.txt:27:toward a dahlia goes to sleep, but a girl about a piroshki always
test/longfile.txt:28:bestows great honor upon a wobbly clodhopper! A girl takes a peek at a
test/longfile.txt:29:bubble living with a ruffian. Some darling toothache trades baseball
test/longfile.txt:30:cards with an impresario. The labyrinth related to the menagé à trois
test/longfile.txt:31:lazily secretly admires the boy beyond a tea party. He called her Lila
test/longfile.txt:32:(or was it Harpo Marx?). Nicolas, although somewhat soothed by the
test/longfile.txt:33:fetishist defined by a marzipan and a clodhopper, still amorously
test/longfile.txt:34:teaches her from a gonad behind an impresario, bestow great honor upon
test/longfile.txt:35:her the lovely fetishist with a cup beyond the pocket, and lazily
test/longfile.txt:36:boogies the dark side of her
test/longfile.txt:37:snow.
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
fn recursive_match_line_numbers_only() {

    let args = "her test --recursive --line-numbers-only";
    let expected_exit_code = 0;
    let expected_screen_output = ["1\n3\n2\n3\n4\n4\n5\n3\n5\n17\n18\n19
21\n22\n24\n25\n31\n34\n35\n36\n1\n3\n4\n"];

    test(&args, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match_no_files_names() {

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

#[test]
fn recursive_match_line_numbers_only_no_match() {

    let args = "her test --recursive --line-numbers-only --no-match";
    let expected_exit_code = 0;
    let expected_screen_output = ["1\n2\n3\n4\n5\n",
                                  "2\n4\n5\n",
                                  "1\n3\n",
                                  "1\n2\n5\n",
                                  "1\n2\n3\n6\n",
                                  "1\n2\n3\n4\n",
                                  "1\n2\n4\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n20\n23\n26\n27\n28\n29\n30\n32\n33\n37\n",
                                  "2\n5\n6\n"];

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
