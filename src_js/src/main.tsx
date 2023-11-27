import './global.css';
import './common.css';
import { createRoot } from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';
import { RecoilRoot } from 'recoil';
import router from './routes'


const dom = document.getElementById('main');
if (dom) {
    createRoot(dom).render(
        <RecoilRoot>
            <RouterProvider router={router} />
        </RecoilRoot>
    );
}