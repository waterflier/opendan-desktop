import { Loader } from 'react-feather'
import { Navigate  } from 'react-router-dom'
import { useLoaderData } from 'react-router-dom'




const Check = () => {
    const data = useLoaderData() as CommonResponse
    console.log('loader', data)
    console.log('loader', data, typeof data)

    // 没有安装docker
    if (data.code == 10010) {
        console.log('no docker, goto install')
        return <Navigate  to='/installing'/>
    }


    return (
        <>
            <div className='flex flex-col items-center justify-between h-[500px]'>
                <h1 className="text-center text-2xl">  Checking...</h1>

                <div className='flex flex-col items-center'>
                    <Loader className='text-dan-blue1 animate-spin' size={60} />
                </div>
            </div>

        </>
    )
}


export default Check