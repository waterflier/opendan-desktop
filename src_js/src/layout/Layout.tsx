import { Outlet } from 'react-router-dom'

const Layout = () => {
    return (
        <>
            <main className='w-[920px] m-auto mt-40 border rounded-lg p-10 min-h-[600px]'>
                <Outlet />
            </main>
        </>
    )
}

export default Layout