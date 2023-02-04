import pytest
from speedireexample.main import build_message

@pytest.mark.asyncio
async def test_main():
    message = await build_message()
    assert(message["message"] == "Hello World")
