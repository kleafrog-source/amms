# tools/vis/server.py
import asyncio
import json
import logging
import os
from fastapi import FastAPI, WebSocket
from fastapi.staticfiles import StaticFiles
from fastapi.responses import HTMLResponse
import uvicorn
import panel as pn
from datetime import datetime

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

app = FastAPI()

# Handle static files
static_dir = "static"
if not os.path.exists(static_dir):
    try:
        os.makedirs(static_dir, exist_ok=True)
        logger.info(f"Created static directory: {os.path.abspath(static_dir)}")
        # Optionally create a simple test file
        with open(os.path.join(static_dir, "test.txt"), "w") as f:
            f.write("Static files are working!")
        app.mount("/static", StaticFiles(directory=static_dir), name="static")
    except Exception as e:
        logger.warning(f"Could not create static directory: {e}")
else:
    app.mount("/static", StaticFiles(directory=static_dir), name="static")

# WebSocket connections
active_connections = set()

@app.websocket("/ws")
async def websocket_endpoint(websocket: WebSocket):
    await websocket.accept()
    active_connections.add(websocket)
    try:
        while True:
            data = await websocket.receive_text()
            # Handle incoming messages
            message = json.loads(data)
            await handle_websocket_message(websocket, message)
    except Exception as e:
        logger.error(f"WebSocket error: {e}")
    finally:
        active_connections.remove(websocket)

async def handle_websocket_message(websocket, message):
    msg_type = message.get("type")
    
    if msg_type == "run_simulation":
        # Import simulation function
        from eqgft_v2_2 import simulate_zitter_experiment
        
        # Run simulation
        results = simulate_zitter_experiment(N_events=10000)
        
        # Send results back
        await websocket.send_json({
            "type": "simulation_results",
            "data": {
                "asymmetry": results["A_meas"],
                "error": results["total_error"],
                "significance": results["significance_vs_QED"]
            }
        })

@app.get("/")
async def get_dashboard():
    return HTMLResponse("""
    <!DOCTYPE html>
    <html>
    <head>
        <title>MMSS Dashboard</title>
        <script src="https://cdn.plot.ly/plotly-2.29.1.min.js"></script>
        <style>
            body { font-family: Arial, sans-serif; margin: 0; padding: 20px; }
            #plot { width: 100%; height: 500px; }
            #runSim {
                padding: 10px 20px;
                font-size: 16px;
                background-color: #4CAF50;
                color: white;
                border: none;
                border-radius: 4px;
                cursor: pointer;
                margin: 10px 0;
            }
            #runSim:hover {
                background-color: #45a049;
            }
            #status {
                margin-top: 10px;
                padding: 10px;
                border-radius: 4px;
                background-color: #f0f0f0;
            }
        </style>
    </head>
    <body>
        <h1>MMSS Dashboard</h1>
        <div id="plot"></div>
        <button id="runSim">Run Simulation</button>
        <div id="status">Status: Ready</div>
        
        <script>
            // Get the current host and port
            const host = window.location.hostname;
            const port = '5007';
            const wsProtocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
            
            // Create WebSocket connection
            const ws = new WebSocket(`${wsProtocol}//${host}:${port}/ws`);
            const plotDiv = document.getElementById('plot');
            const statusDiv = document.getElementById('status');
            
            // Connection opened
            ws.addEventListener('open', (event) => {
                console.log('WebSocket connected');
                statusDiv.textContent = 'Status: Connected';
                statusDiv.style.backgroundColor = '#e8f5e9';
                statusDiv.style.color = '#2e7d32';
            });
            
            // Listen for messages
            ws.addEventListener('message', (event) => {
                try {
                    const data = JSON.parse(event.data);
                    console.log('Received:', data);
                    
                    if (data.type === 'simulation_results') {
                        updatePlot(data.data);
                        statusDiv.textContent = 'Status: Simulation completed';
                    }
                } catch (e) {
                    console.error('Error parsing message:', e);
                }
            });
            
            // Handle errors
            ws.addEventListener('error', (error) => {
                console.error('WebSocket error:', error);
                statusDiv.textContent = 'Status: Connection error';
                statusDiv.style.backgroundColor = '#ffebee';
                statusDiv.style.color = '#c62828';
            });
            
            // Handle disconnection
            ws.addEventListener('close', () => {
                console.log('WebSocket disconnected');
                statusDiv.textContent = 'Status: Disconnected';
                statusDiv.style.backgroundColor = '#fff3e0';
                statusDiv.style.color = '#ef6c00';
            });
            
            // Run simulation button
            document.getElementById('runSim').addEventListener('click', () => {
                statusDiv.textContent = 'Status: Running simulation...';
                statusDiv.style.backgroundColor = '#e3f2fd';
                statusDiv.style.color = '#1565c0';
                
                ws.send(JSON.stringify({
                    type: 'run_simulation',
                    timestamp: new Date().toISOString()
                }));
            });
            
            function updatePlot(data) {
                const trace = {
                    y: [data.asymmetry],
                    type: 'bar',
                    name: 'Asymmetry',
                    error_y: {
                        type: 'data',
                        array: [data.error],
                        visible: true,
                        color: '#666',
                        thickness: 1.5,
                        width: 0
                    },
                    marker: {
                        color: '#4CAF50',
                        line: {
                            color: '#2E7D32',
                            width: 1.5
                        }
                    }
                };
                
                const layout = {
                    title: 'Simulation Results',
                    yaxis: {
                        title: 'Asymmetry',
                        gridcolor: '#f0f0f0'
                    },
                    xaxis: {
                        showticklabels: false
                    },
                    plot_bgcolor: 'white',
                    paper_bgcolor: 'white',
                    margin: { t: 40, r: 30, b: 40, l: 50 },
                    showlegend: false
                };
                
                Plotly.newPlot(plotDiv, [trace], layout, {responsive: true});
            }
        </script>
    </body>
    </html>
    """)

if __name__ == "__main__":
    uvicorn.run("server:app", host="127.0.0.1", port=5007, reload=True)