use strum_macros::{Display, EnumIter, EnumMessage, EnumString};

use crate::movement::{LinePosition, Movement};

#[derive(Display, EnumString, EnumIter, Clone, PartialEq, Debug, EnumMessage)]
pub enum EditCommand {
    #[strum(serialize = "move_line_up")]
    MoveLineUp,
    #[strum(serialize = "move_line_down")]
    MoveLineDown,
    #[strum(serialize = "insert_new_line")]
    InsertNewLine,
    #[strum(serialize = "insert_tab")]
    InsertTab,
    #[strum(serialize = "new_line_above")]
    NewLineAbove,
    #[strum(serialize = "new_line_below")]
    NewLineBelow,
    #[strum(serialize = "delete_backward")]
    DeleteBackward,
    #[strum(serialize = "delete_forward")]
    DeleteForward,
    #[strum(serialize = "delete_forward_and_insert")]
    DeleteForwardAndInsert,
    #[strum(serialize = "delete_word_forward")]
    DeleteWordForward,
    #[strum(serialize = "delete_word_backward")]
    DeleteWordBackward,
    #[strum(message = "Join Lines")]
    #[strum(serialize = "join_lines")]
    JoinLines,
    #[strum(message = "Indent Line")]
    #[strum(serialize = "indent_line")]
    IndentLine,
    #[strum(message = "Outdent Line")]
    #[strum(serialize = "outdent_line")]
    OutdentLine,
    #[strum(message = "Toggle Line Comment")]
    #[strum(serialize = "toggle_line_comment")]
    ToggleLineComment,
    #[strum(serialize = "undo")]
    Undo,
    #[strum(serialize = "redo")]
    Redo,
    #[strum(serialize = "clipboard_copy")]
    ClipboardCopy,
    #[strum(serialize = "clipboard_cut")]
    ClipboardCut,
    #[strum(serialize = "clipboard_paste")]
    ClipboardPaste,
    #[strum(serialize = "yank")]
    Yank,
    #[strum(serialize = "paste")]
    Paste,

    #[strum(serialize = "normal_mode")]
    NormalMode,
    #[strum(serialize = "insert_mode")]
    InsertMode,
    #[strum(serialize = "insert_first_non_blank")]
    InsertFirstNonBlank,
    #[strum(serialize = "append")]
    Append,
    #[strum(serialize = "append_end_of_line")]
    AppendEndOfLine,
    #[strum(serialize = "toggle_visual_mode")]
    ToggleVisualMode,
    #[strum(serialize = "toggle_linewise_visual_mode")]
    ToggleLinewiseVisualMode,
    #[strum(serialize = "toggle_blockwise_visual_mode")]
    ToggleBlockwiseVisualMode,
}

#[derive(Display, EnumString, EnumIter, Clone, PartialEq, Debug, EnumMessage)]
pub enum MoveCommand {
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "right")]
    Right,
    #[strum(serialize = "word_backward")]
    WordBackward,
    #[strum(serialize = "word_forward")]
    WordForward,
    #[strum(serialize = "word_end_forward")]
    WordEndForward,
    #[strum(message = "Document Start")]
    #[strum(serialize = "document_start")]
    DocumentStart,
    #[strum(message = "Document End")]
    #[strum(serialize = "document_end")]
    DocumentEnd,
    #[strum(serialize = "line_end")]
    LineEnd,
    #[strum(serialize = "line_start")]
    LineStart,
    #[strum(serialize = "line_start_non_blank")]
    LineStartNonBlank,
    #[strum(serialize = "go_to_line_default_last")]
    GotoLineDefaultLast,
    #[strum(serialize = "go_to_line_default_first")]
    GotoLineDefaultFirst,
    #[strum(serialize = "match_pairs")]
    MatchPairs,
    #[strum(serialize = "next_unmatched_right_bracket")]
    NextUnmatchedRightBracket,
    #[strum(serialize = "previous_unmatched_left_bracket")]
    PreviousUnmatchedLeftBracket,
    #[strum(serialize = "next_unmatched_right_curly_bracket")]
    NextUnmatchedRightCurlyBracket,
    #[strum(serialize = "previous_unmatched_left_curly_bracket")]
    PreviousUnmatchedLeftCurlyBracket,
}

impl MoveCommand {
    pub fn to_movement(&self, count: Option<usize>) -> Movement {
        use MoveCommand::*;
        match self {
            Left => Movement::Left,
            Right => Movement::Right,
            Up => Movement::Up,
            Down => Movement::Down,
            DocumentStart => Movement::DocumentStart,
            DocumentEnd => Movement::DocumentEnd,
            LineStart => Movement::StartOfLine,
            LineStartNonBlank => Movement::FirstNonBlank,
            LineEnd => Movement::EndOfLine,
            GotoLineDefaultFirst => match count {
                Some(n) => Movement::Line(LinePosition::Line(n)),
                None => Movement::Line(LinePosition::First),
            },
            GotoLineDefaultLast => match count {
                Some(n) => Movement::Line(LinePosition::Line(n)),
                None => Movement::Line(LinePosition::Last),
            },
            WordBackward => Movement::WordBackward,
            WordForward => Movement::WordForward,
            WordEndForward => Movement::WordEndForward,
            MatchPairs => Movement::MatchPairs,
            NextUnmatchedRightBracket => Movement::NextUnmatched(')'),
            PreviousUnmatchedLeftBracket => Movement::PreviousUnmatched('('),
            NextUnmatchedRightCurlyBracket => Movement::NextUnmatched('}'),
            PreviousUnmatchedLeftCurlyBracket => Movement::PreviousUnmatched('{'),
        }
    }
}

#[derive(Display, EnumString, EnumIter, Clone, PartialEq, Debug, EnumMessage)]
pub enum FocusCommand {
    #[strum(serialize = "split_vertical")]
    SplitVertical,
    #[strum(serialize = "split_horizontal")]
    SplitHorizontal,
    #[strum(serialize = "split_exchange")]
    SplitExchange,
    #[strum(serialize = "split_close")]
    SplitClose,
    #[strum(serialize = "split_right")]
    SplitRight,
    #[strum(serialize = "split_left")]
    SplitLeft,
    #[strum(serialize = "split_up")]
    SplitUp,
    #[strum(serialize = "split_down")]
    SplitDown,
    #[strum(serialize = "search_whole_word_forward")]
    SearchWholeWordForward,
    #[strum(serialize = "search_forward")]
    SearchForward,
    #[strum(serialize = "search_backward")]
    SearchBackward,
    #[strum(serialize = "clear_search")]
    ClearSearch,
    #[strum(serialize = "search_in_view")]
    SearchInView,
}

#[derive(Display, EnumString, EnumIter, Clone, PartialEq, Debug, EnumMessage)]
pub enum MotionModeCommand {
    #[strum(serialize = "motion_mode_delete")]
    MotionModeDelete,
    #[strum(serialize = "motion_mode_indent")]
    MotionModeIndent,
    #[strum(serialize = "motion_mode_outdent")]
    MotionModeOutdent,
    #[strum(serialize = "motion_mode_yank")]
    MotionModeYank,
}

#[derive(Display, EnumString, EnumIter, Clone, PartialEq, Debug, EnumMessage)]
pub enum MultiSelectionCommand {
    #[strum(serialize = "select_undo")]
    SelectUndo,
    #[strum(serialize = "insert_cursor_above")]
    InsertCursorAbove,
    #[strum(serialize = "insert_cursor_below")]
    InsertCursorBelow,
    #[strum(serialize = "insert_cursor_end_of_line")]
    InsertCursorEndOfLine,
    #[strum(serialize = "select_current_line")]
    SelectCurrentLine,
    #[strum(serialize = "select_all_current")]
    SelectAllCurrent,
    #[strum(serialize = "select_next_current")]
    SelectNextCurrent,
    #[strum(serialize = "select_skip_current")]
    SelectSkipCurrent,
    #[strum(serialize = "select_all")]
    SelectAll,
}
