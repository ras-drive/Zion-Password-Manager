import React from 'react'
import { useForm } from 'react-hook-form';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import bgImg from '../../assets/logo.svg'

type FormJson = {
    email: String,
    password: String,
    confirmpwd: String,
};

export default function Form() {
    const { register, handleSubmit, watch, formState: { errors } } = useForm()

    const onSubmit = (data: any) => {
        let json = data as FormJson;

        if (json.password !== json.confirmpwd) {
            toast('passwords do not match')

            return;
        }
        
        // creates a fetch request to insert the form into the database through backend user route
        const url = '/api/user';
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
            if (response.status == 200) {
                toast('User successfully registered!\n you can now proceed to the login page')
            } else {
                toast('Error registering user, this is most likely not the user\'s fault')
            }
        });

        };

    return (
        <section>
            <div className='register'>
                <div className='col-1'>
                    <h2>Register new account</h2>
                    <span>register and enjoy the service</span>
    
                    <form id='form' className='flex flex-col' onSubmit={handleSubmit(onSubmit)}>
                        <input type='email' {...register('email')} placeholder='email' />
                        <input type='text' {...register('password')} placeholder='password' />
                        <input type='text' {...register('confirmpwd')} placeholder='confirm password' />
                        <button className='btn'>Register</button>
                    </form>
    
                </div>
                <div className='col-2'>
                    <img src={bgImg} alt='' />
                </div>
                <ToastContainer />
            </div>
        </section>
      )
}