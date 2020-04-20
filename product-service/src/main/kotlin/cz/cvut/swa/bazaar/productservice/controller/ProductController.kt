package cz.cvut.swa.bazaar.productservice.controller

import cz.cvut.swa.bazaar.productservice.data.dto.CreateProductDTO
import cz.cvut.swa.bazaar.productservice.logger
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("products")
class ProductController {

    val log by logger()

    @PostMapping(consumes = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.CREATED)
    fun publishProduct(product: CreateProductDTO) {
        log.debug("> publishProduct - $product")
    }

    @GetMapping("/{id}", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.OK)
    fun getProduct(@PathVariable(name = "id") id: String) {
        log.debug("> getProduct - $id")
    }

    @GetMapping(produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.OK)
    fun getAllProducts() {
        log.debug("> getAllProducts")
    }

    @PostMapping("/update/{id}")
    @ResponseStatus(HttpStatus.OK)
    fun updateProduct(@PathVariable(name = "id") id: String) {
        log.debug("> updateProduct - $id")
    }

    @DeleteMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    fun deleteProduct(@PathVariable(name = "id") id: String) {
        log.debug("> deleteProduct - $id")

    }

}