from fastapi import FastAPI

app = FastAPI()


@app.get("/ping")
def api_ping() -> str:
    return "I am alive"


@app.post("/users")
async def create_user():
    pass
