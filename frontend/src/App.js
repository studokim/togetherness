import { Route, Routes, useNavigate } from 'react-router-dom';
import './App.scss';
import { LoginPage } from './pages/LoginPage/LoginPage';
import MainPage from './pages/MainPage/MainPage';
import FractionPage from './pages/FractionPage/FractionPage';
import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { getCookie } from './helpers/cookies';
import { getPerson, setAvatar, setFraction, setId, setName } from './redux/status';
import { StatusPage } from './pages/StatusPage/StatusPage';

function App() {

  const dispatch = useDispatch();
  const navigator = useNavigate();


  
  useEffect(() => {
    const id = getCookie("togethernessId");
    console.log(id);
    //Если в куках есть id
    if (id !== undefined) {
      dispatch(getPerson({
        id, callback: (name, id, avatarId, fractionId, timer) => {
          dispatch(setName(name));
          dispatch(setId(id));
          dispatch(setFraction(fractionId));
          dispatch(setAvatar(avatarId));
          navigator("/main");
        },
        errorHandler: (error) => {
          console.log("Пользователь не зарегистрирован. ", error);
          navigator("/");
        }
      }));
      console.log("Обновление");
    }
    else {
      navigator("/");
    }
  }, [])

  console.log(process.env.REACT_APP_ADDR);

  return (
    <div className="App">
      <Routes>
        <Route path='*' element={<LoginPage />} />
        <Route path='/*' element={<LoginPage />} />
        <Route path='/fraction' element={<FractionPage />} />
        <Route path='/main' element={<MainPage />} />
        <Route path='/status' element={<StatusPage />} />
      </Routes>
    </div>
  );
}

export default App;