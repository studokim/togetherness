import { Route, Routes, BrowserRouter } from 'react-router-dom';
import './App.scss';
import { LoginPage } from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';
import FractionPage from './pages/FractionPage/FractionPage';
import InteractionPage from './pages/InteractionPage/InteractionPage';


function App() {
  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
          <Route path='/' element={<LoginPage />} />
          <Route path='/fraction' element={<FractionPage />} />
          <Route path='/main' element={<MainPage />} />
          <Route path='/interaction' element={<InteractionPage />} />
          <Route path='/status' element={<LoginPage />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;