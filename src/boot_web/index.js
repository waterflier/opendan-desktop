async function postData() {
    var data = {}
    data["function_name"] = "test"
    //data["params"] = {"a": 1, "b": 2}
    const rawResponse = await fetch('/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    const resp = await rawResponse.json();

    alert(resp.result);
}

async function check_docker_version() {
    const rawResponse = await fetch('/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    const resp = await rawResponse.json();

    alert(resp.result);
}

async function try_install_docker() {
    const rawResponse = await fetch('/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    const resp = await rawResponse.json();

    alert(resp.result);
}

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}



async function exec_long_cmd(cmd_id,cmd,args) {
    var data = {}
    data["function_name"] = "exec_long_cmd"
    data["params"] = {"cmd_id":cmd_id,"cmd": cmd, "args": args}
    const rawResponse = await fetch('/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    const resp = await rawResponse.json();
    return resp;
}

async function get_cmd_new_output(cmd_id) {
    var data = {}
    data["function_name"] = "get_cmd_new_output"
    data["params"] = {"cmd_id":cmd_id}
    const rawResponse = await fetch('/api', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    });

    const resp = await rawResponse.json();

    return resp;
}
//---------------------------------------------------------------------------
async function setup_aios() {
    var docker_version = await check_docker_version();
    if (docker_version < 24) {
        // install docker
        await try_install_docker();
    }
    
    var is_aios_exist = await is_aios_exist();
    if (!is_aios_exist) {
        await update_aios();
    }
    
    await start_aios()
    
    while (true) {
        var check_started = await check_aios()
        if (check_started) {
    
            break;
        }
        await sleep(500);
    
    }

    
    //redirected to aios webui, load complete.
    window.location.replace("http://localhost:9800")
    
}

const submitBtn = document.getElementById('submitBtn');

submitBtn.addEventListener('click', async () => {
    var resp = await exec_long_cmd("test","ping",["8.8.8.8","-n","30"])
    if (resp.error) {
        alert(resp.error);
        return;
    }

    timer_id = 0;
    async function update_exec_logs() {
        const logContent = document.getElementById('logContent');
        const resp = await get_cmd_new_output("test");
        if (resp.result == "") {
            if(resp.error) {
                //long cmd done
                clearInterval(timer_id);
            }
            return;
        }
        logContent.textContent += resp.result;
        const logContainer = document.getElementById('logContainer');
        const isScrolledToBottom = logContainer.scrollHeight - logContainer.clientHeight <= logContainer.scrollTop + 5; 

        if (isScrolledToBottom) {
            logContainer.scrollTop = logContainer.scrollHeight;
        }
        if(resp.error) {
            //long cmd done
            clearInterval(timer_id);
        }
    }

    timer_id = setInterval(update_exec_logs, 200); 
});
