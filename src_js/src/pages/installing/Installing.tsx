import { Loader } from 'react-feather'
import { create } from 'zustand'
// import InstallSuccess from './InstallSuccess'
import { useAsyncEffect } from 'ahooks'
import { docker_download, docker_download_status } from '@services/index'
import { Progress, Space } from 'antd';
import { useState } from 'react'
// import { useLoaderData } from 'react-router-dom'
import _ from 'lodash'

interface DockerStateStoreProps {
    state: number,
    updateState: (state: number) => void,
    // success: () => void,
    // failed: () => void,
}

const useDockerStateStore = create<DockerStateStoreProps>((set) => ({
    state: 0,
    updateState: (state: number) => {
        set({ state })
    },
    // success: () => {
    //     set({ status: 1 })
    // },
    // failed: () => {
    //     set({ status: 2 })
    // }
}))


const Installing = () => {
    const { state, updateState } = useDockerStateStore()
    const [progress, setProgress] = useState(0)

    useAsyncEffect(async () => {
        const data = await docker_download()
        console.log('data', data)
        if (data.result.state == 2) {
            console.log('file exist')
            updateState(data.result.state)
            setProgress(100)
        } else if (data.result.state == 0 || data.result.state == 1) {
            //TODO  progress
            console.log('start download interval')
            updateState(data.result.state)
            let interval = setInterval(async () => {
                console.log('docker_download_status result')
                const result = await docker_download_status()
                console.log('docker_download_status result', result)
                setProgress(parseFloat(result.result.progress))
                if (result.result.state == 2) {
                    clearInterval(interval)
                    console.log('clearInterval')
                }
            }, 1000)
        }
    }, [])


    // if (status == 1) {
    //     return <InstallSuccess />
    // }

    return (
        <>
            <div className='flex flex-col items-center justify-between h-[500px]'>
                <h1 className="text-center text-2xl">  OpenDAN is under installation</h1>
                <div className='flex flex-col items-center '>
                    {/* <Loader className='text-dan-blue1 animate-spin' size={60} /> */}

                    <div className='mt-10 ml-10 w-full'>
                        <Progress percent={progress} strokeColor={{ from: '#108ee9', to: '#87d068' }} />
                    </div>
                    <div>Downloading Docker Desktop</div>

                    {state == 2 && <div className='mt-10'>Installing Docker Desktop</div>}
                </div>
                <div className="flex-center">
                    <div className="btn-dan w-36 h-9">Cancel</div>
                </div>
            </div>

        </>
    )
}


export default Installing