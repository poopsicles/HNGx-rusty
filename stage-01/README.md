# Stage One

<details>
<summary><b>Objectives</b></summary>

Create and host an endpoint using any programming language of your choice.

The endpoint should take two GET request query parameters and return specific information in JSON format.

### The information required includes:

- Slack name
- Current day of the week
- Current UTC time (with validation of +/-2 minutes)
- Track
- The GitHub URL of the file being run
- The GitHub URL of the full source code.
- A Status Code of Success

For example:

```json
{
    "slack_name": "example_name",
    "current_day": "Monday",
    "utc_time": "2023-08-21T15:04:05Z",
    "track": "backend",
    "github_file_url": "https://github.com/username/repo/blob/main/file_name.ext",
    "github_repo_url": "https://github.com/username/repo",
    "status_code": 200
}
```

### Acceptance Criteria

- Endpoint Creation: Provide a publicly accessible endpoint.

- GET Parameters: The endpoint should accept two GET request query parameters: `slack_name` and `track` e.g. `http://example.com/api?slack_name=example_name&track=backend`

- Slack Name: The response should include the `slack_name` passed as a GET request query parameter.

- Current Day of the Week: Display the current day of the week in full (e.g., Monday, Tuesday, etc.).

- Current UTC Time: Return the current UTC time, accurate within a +/-2 minute window.

- Track: The response should display the track of the you signed up for (Backend). This will be based on the track GET parameter passed to the endpoint.

- GitHub File URL: Include a direct link to the specific file in the GitHub repository that's being executed.

- GitHub Repo URL: Include a link to the main page of the GitHub repository containing the project's entire source code.

- Status Code: Return 200 as Integer.

- JSON Format: The endpoint's response should adhere to the specified JSON format.

- Testing: Before submission:
    - Ensure the endpoint is accessible.
    - Check the returned JSON against the defined format.
    - Validate the correctness of each data point in the JSON response.

</details>

Live demo is at [`https://hngx-stage-one-c.onrender.com/api`](https://hngx-stage-one-c.onrender.com/api)

Run it like:

```sh
$ curl -X "GET" "https://hngx-stage-one-c.onrender.com/api?slack_name=example_name&track=backend" -H "accept: application/json"

{
    "slack_name": "example_name",
    "current_day": "Monday",
    "utc_time": "2023-08-21T15:04:05Z",
    "track": "backend",
    "github_file_url": "https://github.com/poopsicles/HNGx-dotnet/blob/main/stage-one/Program.cs",
    "github_repo_url": "https://github.com/poopsicles/HNGx-dotnet",
    "status_code": 200
}   
```

## Build locally

- [Install](https://rustup.rs/) the Rust toolchain

- Clone the repository:

```sh
$ git clone https://github.com/poopsicles/HNGx-rusty.git
$ cd HNGx-rusty/stage-one
```

- Run with cargo

```sh
$ cargo run
```

## Package with Docker

- Clone the repository, and `cd` into `stage-one`

- Build the image:

```sh
$ docker build . --pull -t hngx-uno/rusty
```

- Run the container:

```sh
$ docker run --rm -it -p 8000:80 hngx-uno/rusty
```

It will be running at `http://localhost:8000`