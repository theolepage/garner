# garner

## Usage

Use the following command to upload a file. Its URL will be printed on your terminal.

```
cat file.png | curl --data-binary @- http://garn.er/
```

Type the command below to download the file you just uploaded.

```
curl http://garn.er/the_id_of_my_file > file_received.png
```

## To-Do

- [ ] Add restrictions on files size and type
- [ ] Add a config file (files location, host, port)
- [ ] Put in a Docker container
- [ ] Create a CLI or web client