//Grupo: Antonio Mello Babo, Enzo Maneira, Lucas Benicio Lima, Stefano Giordano

var saveJson = {
  pontos: [],
  retas: [],
  retangulos: [],
  triangulos:[],
  circulos:[]
};


  function _(selector){
    return document.querySelector(selector);
  }

  

  function setup(){
    let canvas = createCanvas(1280, 620);
    canvas.parent("canvas-wrapper");
    background(255);
  }

  function mouseDragged(){
    let type = FindBrushType();
    let size = parseInt(_("#pen-size").value);
    let color = _("#pen-color").value;
    fill(color);
    stroke(color);
    

    switch (type){
      case "pencil":
        color =  _("#pen-color").value;
        line(pmouseX, pmouseY, mouseX, mouseY);
        strokeWeight(7);
        saveJson.pontos.push({
          "nome": "loPuentoFatale",
			    "coord": [[pmouseX, pmouseY], [mouseX, mouseY]],
			    "cor": [color],
        })
        break;
      case "brush":
        color =  _("#pen-color").value;
        ellipse(mouseX, mouseY, size, size);
        saveJson.circulos.push({
          "nome": "loCirculoFatale",
			    "centro": [mouseX,mouseY],
          "raio": size,
			    "cor": [color],
        })
        break;
    }
  }

  function mousePressed() {
    let type = FindBrushType();
    let size = parseInt(_("#pen-size").value);
    let color = _("#pen-color").value;

    if (mouseY > 0 ){
      switch (type){
        case "rectangle":
          color = _("#pen-color").value;
          fill(color);
          stroke(color)
          rectMode(CENTER);
          rect(mouseX, mouseY, 30+(size *3), 30+(size *3));    

          saveJson.retangulos.push({
            "nome": "loRetanguloBendito",
            "pontos": [[mouseX,mouseY], [30+(size *3), 30+(size *3)]],
            "cor": [color]
          })
          break;
        case "triangle":
          color = _("#pen-color").value;
          fill(color);
          stroke(color)
          triangle(mouseX + (size * 2.5), mouseY, mouseX - 85 , mouseY, mouseX-42+ (size * 1.25) , mouseY-50 - (size*2)) ;
          drawing.triangle.push([mouseX + (size * 2.5), mouseY, mouseX - 85 , mouseY, mouseX-42+ (size * 1.25) , mouseY-50 - (size*2)])   
          
          saveJson.triangulos.push({
            "nome": "loTrianguloMaligno",
            "pontos": [[mouseX + (size * 2.5), mouseY], [mouseX - 85 , mouseY], [mouseX-42+ (size * 1.25) , mouseY-50 - (size*2)]],
            "cor": color
          })
          break;
        case "circle":
          color = _("#pen-color").value;
          fill(color);
          stroke(color)
          ellipse(mouseX, mouseY,size + 60,size + 60);
          
          saveJson.circulos.push({
            "nome": "loCirculoFatale",
            "centro": [mouseX,mouseY],
            "raio": size + 60,
            "cor": [color],
          })
          break;
      }
  }

  }

  _("#reset-canvas").addEventListener("click", function(){
    background(255);
     saveJson = {
      pontos: [],
      retas: [],
      retangulos: [],
      triangulos:[],
      circulos:[]
    }
  });
  _("#save-canvas").addEventListener("click",function(){
    saveCanvas(canvas, "sketch", "png");
  });
  _("#save-json").addEventListener("click",function(){
    var dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(saveJson));
    var dlAnchorElem = document.getElementById('downloadAnchorElem');
    dlAnchorElem.setAttribute("href", dataStr);
    dlAnchorElem.setAttribute("download", "backup.json");
    dlAnchorElem.click();

  });

  _("#load-json").addEventListener("click",function(){
    let file = document.querySelector('#file')
    if (!file.value.length) return;
    let reader = new FileReader();
    reader.onload = displayFile;
    reader.readAsText(file.files[0])
  });

  function displayFile (event){
    background(255);
    
    let str = event.target.result;
    let json = JSON.parse(str);

    

    for (let i = 0; i < json.pontos.length; i++){
      strokeWeight(7);
      fill(json.pontos[i].cor);
      stroke(json.pontos[i].cor);
      line(json.pontos[i].coord[0][0], json.pontos[i].coord[0][1], json.pontos[i].coord[1][0], json.pontos[i].coord[1][1]); 
     }

    for (let i = 0; i < json.circulos.length; i++){
      fill(json.circulos[i].cor);
      stroke(json.circulos[i].cor);
      ellipse(json.circulos[i].centro[0], json.circulos[i].centro[1], json.circulos[i].raio, json.circulos[i].raio);   
     }

     for (let i = 0; i < json.retangulos.length; i++){
      fill(json.retangulos[i].cor);
      stroke(json.retangulos[i].cor);
      rect(json.retangulos[i].pontos[0][0], json.retangulos[i].pontos[0][1], json.retangulos[i].pontos[1][0], json.retangulos[i].pontos[1][1]);   
     }

     for (let i = 0; i < json.triangulos.length; i++){
      fill(json.triangulos[i].cor);
      stroke(json.triangulos[i].cor);

      triangle(json.triangulos[i].pontos[0][0], json.triangulos[i].pontos[0][1], json.triangulos[i].pontos[1][0], json.triangulos[i].pontos[1][1], json.triangulos[i].pontos[2][0],json.triangulos[i].pontos[2][1]); 
     }
  }

  function FindBrushType(){
      switch (true) {
        case _("#pen-pencil").checked:
            return "pencil"
        case _("#pen-brush").checked:
            return "brush"
        case _("#triangle").checked:
            return "triangle"
        case _("#rectangle").checked:
              return "rectangle"
        case _("#circle").checked:
          return "circle"
      }

  }
  
  