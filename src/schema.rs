// @generated automatically by Diesel CLI.

diesel::table! {
    programs (id) {
        id -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project_matches (id) {
        id -> Int4,
        createdAt -> Text,
        updatedAt -> Text,
        projectId -> Text,
        roundId -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    project_summaries (id) {
        id -> Int4,
        createdAt -> Text,
        updatedAt -> Text,
        projectId -> Text,
        roundId -> Text,
        chainId -> Text,
        totalVotesInUSDC -> Text,
        uniqueVoters -> Text,
        totalVoters -> Text,
    }
}

diesel::table! {
    projects (id) {
        id -> Text,
        status -> Text,
        payoutAddress -> Nullable<Text>,
        project -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    rounds (id) {
        id -> Text,
        payoutStrategy -> Text,
        token -> Text,
        roundStartTime -> Text,
        roundEndTime -> Text,
        applicationsStartTime -> Text,
        applicationsEndTime -> Text,
        createdAt -> Text,
        updatedAt -> Text,
        chainId -> Nullable<Text>,
    }
}

diesel::table! {
    votes (id) {
        id -> Text,
        createdAt -> Text,
        amount -> Text,
        from -> Text,
        to -> Text,
        token -> Text,
        version -> Text,
        projectId -> Nullable<Text>,
        chainId -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    programs,
    project_matches,
    project_summaries,
    projects,
    rounds,
    votes,
);
