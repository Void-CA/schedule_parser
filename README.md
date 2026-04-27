# schedule_parser
Hice este proyecto porque llevo ya 4 años en la universidad, y nunca arreglaron los horarios mal hechos. Siempre tuve que hacer un proceso manual de copiar y pegar los horarios en un excel, para luego ordenarlos y guardarlos.

Por el momento solo lo hare con los horarios de clases, pero la idea es que eventualmente pueda parsear también los horarios de exámenes, y otros eventos académicos. Todo esto dedicado a la comunidad estudiantil o docente.

## Importante
El proyecto ademas de ser un prototipo, estara desplegado sobre tecnologias de WebAssembly, esto para respetar la privacidad de los usuarios, y evitar que sus datos sean enviados a servidores externos. Por lo tanto, el proyecto se centrara en ser una librería de Rust que pueda ser compilada a WebAssembly, y luego integrada en aplicaciones web.

## Funcionamiento
El proyecto se basa en el uso de expresiones regulares para extraer información relevante de los horarios en formato PDF. El proceso general es el siguiente:
1. Leer el PDF y extraer su contenido como texto.
2. Dividir el texto en líneas para procesarlas individualmente.
3. Utilizar expresiones regulares para identificar patrones específicos, como encabezados de carrera, horarios de clases, etc.
4. Almacenar la información extraída en estructuras de datos organizadas, como structs de Rust.
5. Proporcionar una interfaz para acceder a esta información de manera estructurada, facilitando su uso en aplicaciones web.