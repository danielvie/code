from fastapi import FastAPI
from fastapi.responses import HTMLResponse
import uvicorn

app = FastAPI()

index_html = """
<!DOCTYPE html>
<html>
<head>
<style>
  body { font-family: sans-serif; margin: 20px; }
  #draggable {
    width: 100px;
    height: 100px;
    background-color: #007bff;
    color: white;
    text-align: center;
    line-height: 100px;
    cursor: grab;
    user-select: none;
    border-radius: 8px;
  }
</style>
<script>
  function undock() {
    window.open('/undocked', 'popup', 'width=400,height=400');
  }

  document.addEventListener("DOMContentLoaded", () => {
      const dragItem = document.getElementById("draggable");
      dragItem.addEventListener("dragstart", (e) => {
          e.dataTransfer.setData("text/plain", "dropped_content");
          e.dataTransfer.effectAllowed = "move";
      });
  });
</script>
</head>
<body>
  <h2>Main Page (Window A)</h2>
  <div id="draggable" draggable="true">Drag Me</div>
  <br/>
  <button id="undock" onclick="undock()">Undock</button>
</body>
</html>
"""

undocked_html = """
<!DOCTYPE html>
<html>
<head>
<style>
  body { font-family: sans-serif; margin: 20px; }
  #dropzone {
    width: 200px;
    height: 200px;
    border: 2px dashed #999;
    text-align: center;
    line-height: 200px;
    border-radius: 8px;
    transition: background-color 0.3s;
  }
  .dropped {
    background-color: #d4edda !important;
    border-color: #28a745 !important;
    color: #155724;
  }
</style>
<script>
  document.addEventListener("DOMContentLoaded", () => {
      const dropzone = document.getElementById("dropzone");
      
      dropzone.addEventListener("dragover", (e) => {
          e.preventDefault(); 
          e.dataTransfer.dropEffect = "move";
      });
      
      dropzone.addEventListener("drop", (e) => {
          e.preventDefault();
          const data = e.dataTransfer.getData("text/plain");
          if (data === "dropped_content") {
              dropzone.textContent = "Dropped";
              dropzone.classList.add("dropped");
          }
      });
  });
</script>
</head>
<body>
  <h2>Popup (Window B)</h2>
  <div id="dropzone">Empty</div>
</body>
</html>
"""

@app.get("/")
def get_main():
    return HTMLResponse(index_html)

@app.get("/undocked")
def get_undocked():
    return HTMLResponse(undocked_html)

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
