# Web-check(webc) 
a simple spam web server analyzer written in rust. Creates a huge spam connection at a certain endpoint using the POST or GET method (the rest are not yet available) \
The output shows the total number of requests, successful, failed, and the client's rps per session.

## Usage

```bash
webc [OPTIONS] <URL>
```

### Options

| Short Flag | Long Flag | Description | Default |
| :--- | :--- | :--- | :--- |
| `-t` / `--t` | `-threads` | Number of concurrent worker threads | `1` |
| `-d` / `--d` | `-duration` | Test duration in seconds | `30` |
| `-m` / `--m` | `-method` | HTTP method (`GET` or `POST`) | `GET` |

*Both space-separated (`-t 4`) and equals-separated (`--t=4`) argument formats are supported.*

### Examples

Run a 10-second `GET` benchmark with 4 threads:

```bash
webc --t=4 --d=10 http://127.0.0.1:8080
```

Run a 30-second `POST` load test with 2 threads:

```bash
webc -t 2 -m POST http://127.0.0.1:8080
```

### Sample Output

```text

| all        | oks                  | errors               | rps             | dur          |
|------------|----------------------|----------------------|-----------------|--------------|
| 29205      | 29205 (100.00%)      | 0 (0.00%)            | 29205.0 req/s   | 1s           |
```