import { Route, Routes, BrowserRouter } from 'react-router-dom';
import './App.scss';
import { LoginPage } from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';
import FractionPage from './pages/FractionPage/FractionPage';
import InteractionPage from './pages/InteractionPage/InteractionPage';
import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { getCookie } from './helpers/cookies';
import { getPerson, setAvatar, setFraction, setId, setName } from './redux/status';

function App() {

  const dispatch = useDispatch();
  useEffect(() => {
    const id = getCookie("togethernessId");
    console.log(id);
    //Если в куках есть id
    if (id !== undefined) {
      dispatch(getPerson({
        id, callback: (name, id, avatarId, fractionId) => {
          dispatch(setName(name));
          dispatch(setId(id));
          dispatch(setFraction(fractionId));
          dispatch(setAvatar(avatarId));
        }
      }));
      console.log("Обновление");
    }
  }, [])

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