


export async function check_docker_version() {

    const url = '/api'
    const result = await fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json', // 设置内容类型为 JSON
        },
        body: JSON.stringify({
            function_name: 'check_docker_version'
        })
    })

    const v = await result.json()
    console.log(v)
}