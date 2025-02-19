# «correctortcswm»

Cuando guardas con la extensión SingleFile una copia de una página almacenada en linea por parte de web.archive.org, la marca temporal del archivo corresponderá al momento en el cual fue guardado el archivo, siendo distinta a la marca temporal que está en internet. 

Si el archivo a revisar tiene comentados sus datos de origen:
> Page saved with SingleFile... url: ...

este programa revisa el enlace contenido y de ahí toma la fecha original para renombrar. El formato por defecto es el siguiente, pero se puede cambiar según la siguiente sintaxis: [https://docs.rs/chrono/latest/chrono/format/strftime/index.html](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). Un ejemplo, siendo %7title7 el título de la página tomado de la etiqueta <title>:
> %Y-%m-%dT%H:%M:%SZ - %7title7

Para cambiar esto, el primer argumento debe tener un texto similar al anterior. También es posible omitir ese argumento y poner directamente el archivo a renombrar, y quedaría con el formato predeterminado.
