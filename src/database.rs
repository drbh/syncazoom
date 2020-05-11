use crate::*;

/// Open a sqlite3 connection and insert or replace a meeting
/// We use the SQL commands `INSERT OR REPLACE` in order to overwrite any existing values.
pub fn insert_meeting(meeting: Meeting) -> rusqlite::Result<()> {
    let conn = Connection::open("meetings.sql3")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS [meetings] (
        	[uuid] VARCHAR PRIMARY KEY NOT NULL,
			[duration] VARCHAR NOT NULL,
			[email] VARCHAR NULL,
			[end_time] VARCHAR NOT NULL,
			[has_3rd_party_audio] BOOL NOT NULL,
			[has_pstn] BOOL NOT NULL,
			[has_recording] BOOL NOT NULL,
			[has_screen_share] BOOL NOT NULL,
			[has_sip] BOOL NOT NULL,
			[has_video] BOOL NOT NULL,
			[has_voip] BOOL NOT NULL,
			[host] VARCHAR NOT NULL,
			[id] INT NOT NULL,
			[participants] INT NOT NULL,
			[start_time] VARCHAR NOT NULL,
			[topic] VARCHAR NOT NULL,
			[user_type] VARCHAR NOT NULL
		);",
        params![],
    )?;
    conn.execute(
        "INSERT OR REPLACE INTO meetings VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17);",
        params![
            meeting.uuid,
            meeting.duration,
            meeting.email,
            meeting.end_time,
            meeting.has_3rd_party_audio,
            meeting.has_pstn,
            meeting.has_recording,
            meeting.has_screen_share,
            meeting.has_sip,
            meeting.has_video,
            meeting.has_voip,
            meeting.host,
            meeting.id,
            meeting.participants,
            meeting.start_time,
            meeting.topic,
            meeting.user_type,
        ],
    )?;
    Ok(())
}

/// WARNING: this function is not used currently
/// and the same logic is implemented by the Python sctipt
/// that consumes the data from the DB.
/// Over time that Python script may be ported to this repo
/// Open a sqlite3 connection and drop the table
/// we also need to vacuum up the freed space
pub fn _drop_table_and_clear_memory() -> rusqlite::Result<()> {
    let conn = Connection::open("meetings.sql3")?;
    // lets drop the table
    conn.execute("DROP TABLE meetings;", params![])?;
    // we need to clean up the space
    conn.execute("VACUUM;", params![])?;
    Ok(())
}
