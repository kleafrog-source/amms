const statusEl = document.querySelector('#status');
const metricsEl = document.querySelector('#metrics');
const refreshMetricsBtn = document.querySelector('#refresh-metrics');
const taskListEl = document.querySelector('#task-list');
const submitTaskBtn = document.querySelector('#submit-task');
const taskJsonEl = document.querySelector('#task-json');
const taskExecuteEl = document.querySelector('#task-execute');
const llmQueryBtn = document.querySelector('#send-llm');
const llmQueryEl = document.querySelector('#llm-query');
const llmResponseEl = document.querySelector('#llm-response');
const vizPacketEl = document.querySelector('#viz-packet');
const researchGoalEl = document.querySelector('#campaign-goal');
const researchTargetEl = document.querySelector('#campaign-target');
const researchTargetValueEl = document.querySelector('#campaign-target-value');
const researchStepsEl = document.querySelector('#campaign-steps');
const researchContextEl = document.querySelector('#campaign-context');
const researchResultEl = document.querySelector('#research-result');
const startResearchBtn = document.querySelector('#start-research');

const api = (path, options = {}) => fetch(`/api${path}`, {
  headers: { 'Content-Type': 'application/json', ...(options.headers || {}) },
  ...options,
}).then(async res => {
  if (!res.ok) {
    const body = await res.text();
    throw new Error(body || res.statusText);
  }
  return res.json();
});

function updatePhysicsConstants(metrics) {
  const electronMassEl = document.querySelector('#electron-mass');
  const fineStructureEl = document.querySelector('#fine-structure');
  const coherenceEl = document.querySelector('#quaternion-coherence');
  const windingEl = document.querySelector('#topological-winding');

  if (!metrics || !electronMassEl) return;

  electronMassEl.textContent = `${metrics.emergent_electron_mass.toExponential(10)} kg`;
  fineStructureEl.textContent = metrics.fine_structure_constant.toExponential(10);
  coherenceEl.textContent = metrics.quaternion_coherence.toFixed(6);
  windingEl.textContent = metrics.topological_winding.toFixed(4);
}

async function refreshMetrics() {
  try {
    const data = await api('/metrics');
    metricsEl.textContent = JSON.stringify(data, null, 2);
    updatePhysicsConstants(data.metrics);
  } catch (err) {
    metricsEl.textContent = `Error: ${err.message}`;
  }
}

async function refreshTasks() {
  try {
    const tasks = await api('/tasks');
    taskListEl.innerHTML = '';
    tasks.forEach(task => {
      const tr = document.createElement('tr');
      tr.innerHTML = `<td>${task.task_id}</td><td>${JSON.stringify(task.status)}</td>`;
      taskListEl.appendChild(tr);
    });
  } catch (err) {
    taskListEl.innerHTML = `<tr><td colspan="2">${err.message}</td></tr>`;
  }
}

async function refreshViz() {
  try {
    const data = await api('/visualization/packet');
    vizPacketEl.textContent = JSON.stringify(data, null, 2);
  } catch (err) {
    vizPacketEl.textContent = `Error: ${err.message}`;
  }
}

async function refreshHopfionField() {
    try {
        const data = await api('/visualization/hopfion-field');
        if (data) {
            const x = data.q_x.map(v => v[1]);
            const y = data.q_x.map(v => v[2]);
            const z = data.q_x.map(v => v[3]);
            const u = data.q_x.map(v => v[1]);
            const v = data.q_x.map(v => v[2]);
            const w = data.q_x.map(v => v[3]);

            Plotly.newPlot('eqgft-response', [{
                type: 'cone',
                x, y, z, u, v, w,
                sizemode: 'absolute',
                sizeref: 1,
            }]);
        }
    } catch (err) {
        eqgftResponseEl.textContent = `Error: ${err.message}`;
    }
}

submitTaskBtn?.addEventListener('click', async () => {
  try {
    const parsed = JSON.parse(taskJsonEl.value);
    await api('/tasks', {
      method: 'POST',
      body: JSON.stringify({ task: parsed, execute: taskExecuteEl.checked }),
    });
    statusEl.textContent = 'Task submitted.';
    refreshTasks();
    refreshMetrics();
  } catch (err) {
    statusEl.textContent = `Task error: ${err.message}`;
  }
});

startResearchBtn?.addEventListener('click', async () => {
  const goal = researchGoalEl?.value.trim() || 'Достичь topological_winding = 9.0000';
  const optimizationTarget = researchTargetEl?.value.trim() || 'topological_winding';
  const targetValue = parseFloat(researchTargetValueEl?.value || '0') || 0;
  const maxSteps = parseInt(researchStepsEl?.value || '5', 10) || 5;
  let contextPayload = {};

  if (researchContextEl?.value.trim()) {
    try {
      contextPayload = JSON.parse(researchContextEl.value);
    } catch (err) {
      researchResultEl.textContent = `Invalid context JSON: ${err.message}`;
      return;
    }
  }

  researchResultEl.textContent = 'Running research campaign…';

  try {
    const response = await api('/llm/research-campaign', {
      method: 'POST',
      body: JSON.stringify({
        goal,
        optimization_target: optimizationTarget,
        target_value: targetValue,
        max_steps: maxSteps,
        context: contextPayload,
      }),
    });

    researchResultEl.textContent = JSON.stringify(response, null, 2);
    statusEl.textContent = 'Research campaign completed.';
    refreshMetrics();
  } catch (err) {
    researchResultEl.textContent = `Campaign error: ${err.message}`;
  }
});

refreshMetricsBtn?.addEventListener('click', () => {
  refreshMetrics();
  refreshViz();
});

llmQueryBtn?.addEventListener('click', async () => {
  try {
    const result = await api('/llm/query', {
      method: 'POST',
      body: JSON.stringify({ query: llmQueryEl.value }),
    });
    llmResponseEl.textContent = JSON.stringify(result, null, 2);
    taskJsonEl.value = JSON.stringify(result, null, 2);
    statusEl.textContent = 'LLM responded with task blueprint.';
  } catch (err) {
    llmResponseEl.textContent = err.message;
  }
});

refreshMetrics();
refreshTasks();
refreshViz();
statusEl.textContent = 'Ready';

const eqgftVizTypeEl = document.querySelector('#eqgft-viz-type');
const eqgftPythonScriptContainerEl = document.querySelector('#eqgft-python-script-container');
const eqgftRunBtn = document.querySelector('#eqgft-run');
const eqgftSendLlmBtn = document.querySelector('#eqgft-send-llm');
const eqgftLlmQueryEl = document.querySelector('#eqgft-llm-query');
const eqgftResponseEl = document.querySelector('#eqgft-response');

eqgftVizTypeEl?.addEventListener('change', () => {
    if (eqgftVizTypeEl.value === 'custom-python-script') {
        eqgftPythonScriptContainerEl.style.display = 'block';
    } else {
        eqgftPythonScriptContainerEl.style.display = 'none';
    }
});

eqgftRunBtn?.addEventListener('click', async () => {
    const vizType = eqgftVizTypeEl.value;
    const nEvents = document.querySelector('#eqgft-n-events').value;
    const kappa = document.querySelector('#eqgft-kappa').value;
    const systematicError = document.querySelector('#eqgft-systematic-error').value;
    const pythonScript = document.querySelector('#eqgft-python-script').value;

    let task;
    if (vizType === 'custom-python-script') {
        task = {
            task_name: 'Custom Python Script',
            geometric_operator: 'CustomPythonScript',
            parameters: {
                script: pythonScript,
            },
        };
    } else if (vizType === 'line-plot') {
        task = {
            task_name: 'Simulate EQGFT Asymmetry',
            geometric_operator: 'SimulateEqgftAsymmetry',
            parameters: {
                kappa: parseFloat(kappa),
                n_events: parseInt(nEvents),
                systematic_error: parseFloat(systematicError),
            },
        };
    } else {
        task = {
            task_name: 'Generate Hopfion Field',
            geometric_operator: 'GenerateHopfionField',
            parameters: {},
        };
    }

    try {
        await api('/tasks', {
            method: 'POST',
            body: JSON.stringify({ task: task, execute: true }),
        });
        statusEl.textContent = 'EQGFT task submitted.';
        refreshTasks();
        refreshMetrics();
        if (vizType === '3d-vector-field') {
            refreshHopfionField();
        }
    } catch (err) {
        statusEl.textContent = `EQGFT task error: ${err.message}`;
    }
});

eqgftSendLlmBtn?.addEventListener('click', async () => {
    try {
        const task = await api('/llm/plan-eqgft-task', {
            method: 'POST',
            body: JSON.stringify({ query: eqgftLlmQueryEl.value }),
        });
        eqgftResponseEl.textContent = JSON.stringify(task, null, 2);
        statusEl.textContent = 'LLM responded with EQGFT task blueprint. Submitting task...';

        const { task_id } = await api('/tasks', {
            method: 'POST',
            body: JSON.stringify({ task: task, execute: true }),
        });

        const poll = setInterval(async () => {
            const status = await api(`/tasks/${task_id}`);
            if (status.status === 'Completed') {
                clearInterval(poll);
                statusEl.textContent = 'EQGFT task completed.';
                refreshMetrics();
                refreshViz();
            }
        }, 1000);
    } catch (err) {
        eqgftResponseEl.textContent = err.message;
    }
});
