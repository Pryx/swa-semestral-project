package cz.cvut.swa.bazaar.productservice.controller

import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.data.*
import cz.cvut.swa.bazaar.productservice.logger
import cz.cvut.swa.bazaar.productservice.service.ProductService
import org.springframework.http.HttpStatus
import org.springframework.http.MediaType
import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("product")
class ProductController(
        val objectMapper: ObjectMapper,
        val productRepository: ProductRepository,
        val productService: ProductService
) {

    val log by logger()

    @PostMapping(consumes = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.CREATED)
    fun publishProduct(@RequestBody productToCreate: ProductDTO): Product {
        log.debug("> publishProduct - $productToCreate")

        val product = objectMapper.convertValue(productToCreate, Product::class.java)

        val savedProduct = productRepository.save(product)

        log.debug("< publishProduct - created - ${savedProduct.id}")
        return savedProduct
    }

    @GetMapping("/{id}", produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.OK)
    fun getProduct(@PathVariable(name = "id") id: String): ProductWithReviewsDTO {
        log.debug("> getProduct - $id")

        val productWithReviewsDTO = productService.getProductWithReviews(id)

        log.debug("< getProduct - $productWithReviewsDTO")
        return productWithReviewsDTO
    }

    @GetMapping(produces = [MediaType.APPLICATION_JSON_VALUE])
    @ResponseStatus(HttpStatus.OK)
    fun getAllProducts(): MutableList<Product> {
        log.debug("> getAllProducts")

        val productList = productRepository.findAll()

        log.debug("< getAllProducts - $productList")
        return productList
    }

    @PostMapping("/update")
    @ResponseStatus(HttpStatus.OK)
    fun updateProduct(@RequestBody product: Product): Product {
        log.debug("> updateProduct - $product")

        val updatedProduct = productRepository.save(product)

        log.debug("< updateProduct - $updatedProduct")
        return updatedProduct
    }

    @PostMapping("/update/status")
    @ResponseStatus(HttpStatus.OK)
    fun updateProductStatus(@RequestBody statusUpdate: ProductStatusUpdateDTO): Product {
        log.debug("> updateProductStatus - $statusUpdate")

        val product = productRepository.findById(statusUpdate.id)
                .orElseThrow { EntityNotFoundException("Failed to find product") }

        product.status = statusUpdate.newStatus
        val updatedProduct = productRepository.save(product)

        log.debug("< updateProductStatus - $updatedProduct")
        return updatedProduct
    }

    @DeleteMapping("/{id}")
    @ResponseStatus(HttpStatus.OK)
    fun deleteProduct(@PathVariable(name = "id") id: String) {
        log.debug("> deleteProduct - $id")

        productRepository.deleteById(id)
    }

}