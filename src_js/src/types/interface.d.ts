interface CommonResponse<T> {
    code: number
    result: T
    error: string
}

interface DockerDownloadResonse {
    message: string
    state: number
}

interface DockerDownloadStatusResonse {
    message: string
    state: number
    progress: string
}