import { createBrowserRouter } from 'react-router-dom'
import Layout from '@layout/Layout'
import Index from '@pages/index/Index'
import Installed from '@pages/installed/Installed'
import Installing from '@pages/installing/Installing'
import Check from '@pages/check/Check'

const router = createBrowserRouter([
    {
        path: '/',
        element: <Layout />,
        children: [
            {
                path: '/',
                element: <Index />,

            },
            {
                path: '/installed',
                element: <Installed />
            },
            {
                path: '/check',
                element: <Check />,
                loader: async () => {
                    return fetch(`/api/docker/check`, {method: 'POST'});
                }
            },
            {
                path: '/installing',
                element: <Installing />,
            },
        ]
    }
]);

export default router;
