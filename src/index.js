import React from 'react';
import ReactDOM from 'react-dom';
import { MainPage, RegistrationPage } from './src/components/pages.js'; 

const App = () => {
    return (
        <div>
            <MainPage />
            <RegistrationPage />
        </div>
    );
}

const root = document.createElement('div');
document.body.appendChild(root);
ReactDOM.createRoot(root).render(<App />);
