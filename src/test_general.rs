/// Just a few general tests. The specifics are tested in the other test files.

use ned;

#[test]
fn basic_match() {

    let options = "accidentally test";
    let expected_exit_code = 0;
    let expected_screen_output = "The accidentally ghastly hand plans an escape from a cream puff \
                                  the placid widow. A slovenly\nonlooker rejoices, because some \
                                  single-handledly sheepish stalactite knowingly avoids contact \
                                  with a\nwisely rhetorical ballerina. Sometimes the waif about a \
                                  swamp rejoices, but a ruffian always barely\nbefriends an \
                                  unseemly dilettante! Unlike so many mastadons who have made \
                                  their lovely widow\nabhorrent to us, waifs remain womanly.";

    test(&options, expected_exit_code, &expected_screen_output);
}

#[test]
fn only_matches() {

    let options = "accidentally.*hand test --only-matches";
    let expected_exit_code = 0;
    let expected_screen_output = "accidentally ghastly hand";

    test(&options, expected_exit_code, &expected_screen_output);
}

#[test]
fn colored_match() {

    let options = "accidentally.*hand test --colors";
    let expected_exit_code = 0;
    let expected_screen_output = "The \u{1b}[1;31maccidentally ghastly hand\u{1b}[0m plans an \
                                  escape from a cream puff the placid widow. A slovenly\nonlooker \
                                  rejoices, because some single-handledly sheepish stalactite \
                                  knowingly avoids contact with a\nwisely rhetorical ballerina. \
                                  Sometimes the waif about a swamp rejoices, but a ruffian always \
                                  barely\nbefriends an unseemly dilettante! Unlike so many \
                                  mastadons who have made their lovely widow\nabhorrent to us, \
                                  waifs remain womanly.";

    test(&options, expected_exit_code, &expected_screen_output);
}

#[test]
fn recursive_match() {

    let options = "her test -R";
    let expected_exit_code = 0;
    let expected_screen_output = "The shadow conquers the hand related to a mastadon. Jespera and \
                                  I took a cup around a toothache\n(with a lunatic around some \
                                  debutante, a ribbon beyond a curse, a few dahlias, and a \
                                  ribbon) to\narrive at a state of intimacy where we can \
                                  accurately mourn our boy. When another espadrille wakes\nup, \
                                  the cup toward another swamp flies into a rage. Now and then, \
                                  an onlooker sells a dissident\nrelated to the hand to an \
                                  ungodly dahlia.A bicep near the mastadon has a change of heart \
                                  about a lovely snow. A halfhearted curse steals\npencils from \
                                  the maestro. He called her the Interloper (or was it Timosha?). \
                                  Now and then, the\ntrombone almost gives secret financial aid \
                                  to the wisely strawberry-blonde marzipan.The omphalos toward a \
                                  bubble bath is lowly. The unsightly bicep panics, and a \
                                  toothpick feels\nnagging remorse; however, a bubble toward a \
                                  bonbon pees on an almost ghastly lunatic. Desdemona,\nalthough \
                                  somewhat soothed by a rapacious trombone and the girl beyond a \
                                  gypsy, still bestows great\nhonor upon her from another \
                                  onlooker beyond the maestro, bestow great honor upon her a tea \
                                  party\nwith a bubble, and sells a bonbon to the dark side of \
                                  her looking glass. If the halfhearted waif\nboogies a \
                                  menag\u{e9} \u{e0} trois, then some bride beams with joy.An \
                                  onlooker, a curmudgeonly swamp, and a bubble are what got \
                                  Scheherazade into trouble. A slyly\nunruffled dissident is \
                                  underhandedly curmudgeonly. Harpo Marx and I took a coward \
                                  (with a guardian\nangel living with a cream puff, the bubble \
                                  beyond a swamp, a few dahlias, and another starlet over \
                                  a\nbonbon) to arrive at a state of intimacy where we can wisely \
                                  write a love letter to our taxidermist.\nThe taxidermist goes \
                                  to sleep, and some mirror flies into a rage; however, the \
                                  guardian angel\nrelated to some dilettante makes a truce with \
                                  the lowly labyrinth.If another ballerina defined by a fetishist \
                                  falls in love with a shadow, then a \
                                  non-chalantly\nself-actualized taxidermist prays. Sometimes a \
                                  clodhopper near the hand dies, but the girl around\nsome \
                                  piroshki always operates a small fruit stand with the friendly \
                                  toothache! Nimbo and I took the\nsingle-handledly niggardly \
                                  cream puff (with the trombone, the stalactite, a few midwifes, \
                                  and\nanother onlooker) to arrive at a state of intimacy where \
                                  we can greedily bestow great honor upon our\npiroshki.A bicep \
                                  near the mastadon has a change of heart about a lovely snow. A \
                                  halfhearted curse steals\npencils from the maestro. He called \
                                  her the Interloper (or was it Timosha?). Now and then, \
                                  the\ntrombone almost gives secret financial aid to the wisely \
                                  strawberry-blonde marzipan.Harpo Marx and I took another hand \
                                  for some espadrille (with an irreconcilable tea party, a \
                                  widow\nliving with the fetishist, a few midwifes, and a \
                                  clodhopper near some clock) to arrive at a state of\nintimacy \
                                  where we can carelessly slyly organize our necromancer. Another \
                                  nefarious bubble teaches a\nseldom comely necromancer. A coward \
                                  gives a pink slip to a cleavage toward a bicep. The girl near \
                                  a\ntoothpick shares a shower with the ballerina.";

    test(&options, expected_exit_code, &expected_screen_output);
}

fn test(options: &str, expected_exit_code: i32, expected_screen_output: &str) {
    let program = "ned";
    let args: Vec<String> = options.split_whitespace()
                                   .map(|arg| arg.to_string())
                                   .collect::<Vec<String>>();
    let mut screen_output: Vec<u8> = vec![];

    let exit_code = ned(&program, &args, &mut screen_output).unwrap();

    let screen_output = String::from_utf8(screen_output).unwrap();

    assert_eq!(exit_code, expected_exit_code);
    assert_eq!(screen_output, expected_screen_output);
}
