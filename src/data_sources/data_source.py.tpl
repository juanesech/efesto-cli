import os


class {name}DS:
    def __init__(self, url="https://gitlab.com", token=None):
        self.url = os.getenv("GITLAB_URL") or url
        if url is None:
            raise ValueError("GITLAB_URL is not set")
        self.token = token or os.getenv("GITLAB_TOKEN")
        if token is None:
            raise ValueError("GITLAB_TOKEN is not set")
        self.glab = gitlab.Gitlab(url, token)
