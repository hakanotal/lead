from locust import HttpUser, task

# locust -f locustfile.py -H http://127.0.0.1:8080

class TestUser(HttpUser):

    @task
    def hello_world(self):
        self.client.get("/leaderboard")
