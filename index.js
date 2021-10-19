//Grupo: Antonio Mello Babo, Enzo Maneira, Lucas Benicio Lima, Stefano Giordano

var drawing = {
  points: [],
  rect: [],
  triangle:[],
  ellipse:[]
};


  function _(selector){
    return document.querySelector(selector);
  }

  function setup(){
    let canvas = createCanvas(1250, 600);
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
        line(pmouseX, pmouseY, mouseX, mouseY);
        strokeWeight(7);
        drawing.points.push([pmouseX, pmouseY, mouseX, mouseY]);
        break;
      case "brush":
        ellipse(mouseX, mouseY, size, size);
        drawing.ellipse.push([pmouseX, pmouseY, size, size]);
        break;
    }
  }

  function mousePressed() {
    let type = FindBrushType();
    let size = parseInt(_("#pen-size").value);
    let color = _("#pen-color").value;

    switch (type){
      case "rectangle":
        rectMode(CENTER);
        rect(mouseX, mouseY, 30+(size *3), 50+(size *3));    
        drawing.rect.push([mouseX, mouseY, 50, 50])   
        break;
      case "triangle":
        triangle(mouseX + (size * 2.5), mouseY, mouseX - 85 , mouseY, mouseX-42+ (size * 1.25) , mouseY-50 - (size*2)) ;
        drawing.triangle.push([mouseX + (size * 2.5), mouseY, mouseX - 85 , mouseY, mouseX-42+ (size * 1.25) , mouseY-50 - (size*2)])   
        break;
      case "circle":
        fill(color);
        stroke(color)
        ellipse(mouseX, mouseY,size + 60,size + 60);
        drawing.ellipse.push([pmouseX, pmouseY, size, size]);
        break;
    }

  }

  _("#reset-canvas").addEventListener("click", function(){
    background(255);
    drawing = {
      points: [],
      rect: [],
      triangle:[]
    }
  });
  _("#save-canvas").addEventListener("click",function(){
    saveCanvas(canvas, "sketch", "png");
  });
  _("#save-json").addEventListener("click",function(){
    var dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(drawing));
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

    for (let i = 0; i < json.points.length; i++){
      let color = _("#pen-color").value;
      fill(color);
      stroke(color);
      line(json.points[i][0], json.points[i][1], json.points[i][2], json.points[i][3]); 
     }

    for (let i = 0; i < json.rect.length; i++){
        rect(json.rect[i][0], json.rect[i][1], json.rect[i][2], json.rect[i][3]);   
     }

     for (let i = 0; i < json.triangle.length; i++){
      triangle(json.triangle[i][0], json.triangle[i][1], json.triangle[i][2], json.triangle[i][3], json.triangle[i][4],json.triangle[i][5]); 
     }

     for (let i = 0; i < json.ellipse.length; i++){
      ellipse(json.ellipse[i][0], json.ellipse[i][1], json.ellipse[i][2], json.ellipse[i][3]); 
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
  
  