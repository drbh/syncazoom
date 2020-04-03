# syncazoom

Zoom is an imporant tool for online communication. Especially in these times of social distancing

This tool is made to help you manage API calls to Zoom and make it easy to store a local copy of Zoom meeting metrics.

### Add your keys

copy the example file and rename it to `config.toml`
```toml
[creds]
key = "YOUR-KEY"
secret = "YOUR-SECRET"

[settings]
seconds_between_calls = 60
cron_interval = "0/10 * * * * *"
```

### Install
```bash
# install rust
cargo install syncazoom
```


### Run app
```bash
syncazoom -c config.toml
```


### Example Output
```
03/04/2020 08:57:00
Start run
                                                                                                     
                                                                                                      
 .oooo.o oooo    ooo ooo. .oo.    .ooooo.   .oooo.     oooooooo  .ooooo.   .ooooo.  ooo. .oo.  .oo.   
d88(  "8  `88.  .8'  `888P"Y88b  d88' `"Y8 `P  )88b   d'""7d8P  d88' `88b d88' `88b `888P"Y88bP"Y88b  
`"Y88b.    `88..8'    888   888  888        .oP"888     .d8P'   888   888 888   888  888   888   888  
o.  )88b    `888'     888   888  888   .o8 d8(  888   .d8P'  .P 888   888 888   888  888   888   888  
8""888P'     .8'     o888o o888o `Y8bod8P' `Y888""8o d8888888P  `Y8bod8P' `Y8bod8P' o888o o888o o888o 
         .o..P'                                                                                       
         `Y8P'                                                                                        

URL 		 	| https://api.zoom.us/v2/metrics/meetings?type=past&page_size=300
Total Estimated Runtime 50 mins


─── Log
°
├── Start 	 	 | 2020-04-02
├── End 	 	 | 2020-04-03
├── Pages 	 	 | 50
├── Current 	 | 1
├── Per Page 	 | 300
├── Total 	 	 | 14938
└── Remaining 	 | 14888
°
├── 3vjYIIsiRbOB9hjeluH6mA==
├── Alice Human's Zoom Meeting
├── Alice Human
├── 2020-04-02T00:00:01Z
└── 2020-04-02T00:11:04Z
°
├── XCjX59nUS2W0rz+5zllm0g==
├── Bob Person's Zoom Meeting
├── Bob Person
├── 2020-04-02T00:40:38Z
└── 2020-04-02T02:04:48Z
°
└── Next Page Token: "k89o4smQsnuOGt03Z0h57EU1u3vYe0GU9a2"
```

### How it works

An essential concept of syncazoom is it's heartbeat. At some interval, it will check if it is currently fetching data - and if not, it will start the process. 

The process makes a call to Zoom's metrics API, and then it stores the data in a local instance of sqlite3. We only "insert or replace" on each uuid so we never have duplicate entries.

### Query
```bash
sqlite3 meetings.sql3 
# SQLite version 3.28.0 2019-04-15 14:49:49
# Enter ".help" for usage hints.
# sqlite> 
```

Then enter the following (or more useful) SQL
```sql
SELECT * FROM meetings LIMIT 10;
```
