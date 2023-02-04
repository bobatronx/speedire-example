from fastapi import FastAPI

app = FastAPI()


@app.get("/hello")
async def root():
    return build_message()

async def build_message():
    return {"message": "Hello World"}