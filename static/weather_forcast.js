// script.js
window.addEventListener('DOMContentLoaded', () => {
    const locationElement = document.getElementById('location');
    const temperatureElement = document.getElementById('temperature');
    const descriptionElement = document.getElementById('description');
  
    // Fetch weather data from API and update HTML elements
    fetch('https://api.openweathermap.org/data/2.5/weather?q=New%20York&appid=YOUR_API_KEY')
      .then(response => response.json())
      .then(data => {
        locationElement.textContent = data.location.name;
        temperatureElement.textContent = `Temperature: ${data.current.temp_c}°C`;
        descriptionElement.textContent = `Description: ${data.current.condition.text}`;
      })
      .catch(error => {
        console.error('Error:', error);
      });
  });