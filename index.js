import('./pkg')
  .then((wasm) => {
    const latitude_field = document.getElementById("custom-lat");
    const longitude_field = document.getElementById("custom-long");
    const zoom_field = document.getElementById("zoominput");

    var current_latitude = 0.0;
    var current_longitude = 0.0;
    var current_zoom = 0.95;
    zoom_field.value = current_zoom;
    latitude_field.value = current_latitude;
    longitude_field.value = current_longitude;
    const redraw = () => {
        latitude_field.value = current_latitude;
        longitude_field.value = current_longitude;
        wasm.draw(current_longitude, current_latitude, current_zoom);
    };

    zoom_field.addEventListener('change', ()=> {
      current_zoom = zoom_field.value;
      redraw();
    });
    latitude_field.addEventListener('change', ()=> {
      current_latitude = latitude_field.value;
      redraw();
    });
    longitude_field.addEventListener('change', ()=> {
      current_longitude = longitude_field.value;
      redraw();
    });
    document.getElementById('button-me').addEventListener('click', () => {
      navigator.geolocation.getCurrentPosition(pos =>{
        console.log(pos);
        current_latitude = pos.coords.latitude;
        current_longitude = pos.coords.longitude;
        redraw();
      }, err=> {console.log("Error getting position")})
    });
    document.getElementById("button-santa").addEventListener('click', ()=>{
      console.log("santa button");
      current_latitude = 90.0;
      current_longitude = 0.0;
      redraw();
    })
    document.getElementById("button-cyprus").addEventListener('click', ()=>{
      console.log("cyrpus button");
      current_latitude = 35.12;
      current_longitude = 33.42;
      redraw();
    })
    document.getElementById("button-capehorn").addEventListener('click', ()=>{
      console.log("cape horn button");
      current_latitude = -55;
      current_longitude = -67;
      redraw();
    })
    document.getElementById("button-new-york").addEventListener('click', ()=>{
      console.log("new-york button");
      current_latitude = 40.71;
      current_longitude = -74.00;
      redraw();
    })
    document.getElementById("button-beijing").addEventListener('click', ()=>{
      console.log("beijing button");
      current_latitude = 39.90;
      current_longitude = 116.39;
      redraw();
    })
    document.getElementById("button-paris").addEventListener('click', ()=>{
      console.log("paris button");
      current_latitude = 48.85;
      current_longitude = 2.34;
      redraw();
    })
    redraw();
    
    document.getElementById("loading-message").textContent="";
    console.log("Module loaded in js");
  })
  .catch(console.error);
