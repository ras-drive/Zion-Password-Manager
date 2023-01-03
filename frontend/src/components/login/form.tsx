import { useForm } from 'react-hook-form';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import bgImg from '../../assets/logo.svg'

type FormJson = {
    email: String,
    password: String
};

export default function Form() {
    const { register, handleSubmit, watch, formState: { errors } } = useForm()

    const onSubmit = (data: any) => {
        let json = data as FormJson;

        const url = '/api/login/user';
        const options = {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json;charset=UTF-8'
        },
        body: JSON.stringify({
            email: json.email,
            password: json.password
        })
        };

        fetch(url, options)
        .then(response => {
            if (response.status == 204) {
                toast('successfully logged in!')
            } else if (response.status == 401) {
                toast('invalid email or password')
            }
        });
        };

    return (
        <section>
            <div className='register'>
                <div className='col-1'>
                    <h2>Login to your account</h2>
                    <span>Login and enjoy the service</span>
                    
                    <form id='get' className='flex flex-col' onSubmit={handleSubmit(onSubmit)}>
                        <input type='email' {...register('email')} placeholder='email' />
                        <input type='text' {...register('password')} placeholder='password' />
                        <button className='btn'>Sign In</button>
                    </form>
    
                </div>
                <div className='col-2'>
                    <img src={bgImg} alt='' />
                </div>
            </div>
            <ToastContainer />
        </section>
      )
}