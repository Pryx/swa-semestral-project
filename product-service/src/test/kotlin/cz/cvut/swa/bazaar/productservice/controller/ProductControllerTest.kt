package cz.cvut.swa.bazaar.productservice.controller

import com.fasterxml.jackson.databind.ObjectMapper
import cz.cvut.swa.bazaar.productservice.*
import cz.cvut.swa.bazaar.productservice.data.*
import cz.cvut.swa.bazaar.productservice.service.ProductService
import org.junit.Assert
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.ArgumentMatchers
import org.mockito.Mockito.`when`
import org.mockito.Mockito.any
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest
import org.springframework.boot.test.mock.mockito.MockBean
import org.springframework.http.MediaType
import org.springframework.test.context.junit4.SpringRunner
import org.springframework.test.web.servlet.MockMvc
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders.get
import org.springframework.test.web.servlet.request.MockMvcRequestBuilders.post
import org.springframework.test.web.servlet.result.MockMvcResultMatchers.status
import java.util.*

@WebMvcTest(ProductController::class)
@RunWith(SpringRunner::class)
class ProductControllerTest {

    @Autowired
    private lateinit var mockMvc: MockMvc

    @Autowired
    private lateinit var objectMapper: ObjectMapper

    @MockBean
    private lateinit var productRepository: ProductRepository

    @MockBean
    private lateinit var productService: ProductService

    @Test
    fun `publishProduct - should convert value, should save product`() {
        val productToCreate = fakeProductDTO()
        val product = fakeProduct()

        `when`(productRepository.save(any(Product::class.java))).thenReturn(product)

        mockMvc.perform(
                post("/products")
                        .contentType(MediaType.APPLICATION_JSON_VALUE)
                        .content(objectMapper.writeValueAsString(productToCreate)))
                .andExpect(status().isCreated)
    }

    @Test
    fun `getProduct - product does not exist, should return error`() {
        val productId = randomUuid()

        `when`(productService.getProductWithReviews(productId)).thenThrow(EntityNotFoundException::class.java)

        mockMvc.perform(
                get("/products/{id}", productId))
                .andExpect(status().isNotFound)
    }

    @Test
    fun `getProduct - failed to call service, should return error`() {
        val productId = randomUuid()

        `when`(productService.getProductWithReviews(productId)).thenThrow(NetworkException::class.java)

        mockMvc.perform(
                get("/products/{id}", productId))
                .andExpect(status().isServiceUnavailable)
    }

    @Test
    fun `getProduct - product found, should return product`() {
        val productId = randomUuid()
        val productWithReviews = fakeProductWithReviewsDTO()

        `when`(productService.getProductWithReviews(productId)).thenReturn(productWithReviews)

        val mvcResult = mockMvc.perform(
                get("/products/{id}", productId))
                .andExpect(status().isOk)
                .andReturn()

        val responseString = mvcResult.response.contentAsString
        val fetchedProduct = objectMapper.readValue(responseString, ProductWithReviewsDTO::class.java)
        Assert.assertNotNull(fetchedProduct)
    }

    @Test
    fun `updateProductStatus - product not found, should return error`() {
        val productUpdate = fakeProductStatusUpdateDTO(ProductStatus.SOLD)

        `when`(productRepository.findById(productUpdate.id)).thenReturn(Optional.empty())

        mockMvc.perform(
                post("/products/update/status")
                        .contentType(MediaType.APPLICATION_JSON_VALUE)
                        .content(objectMapper.writeValueAsString(productUpdate)))
                .andExpect(status().isNotFound)
    }

    @Test
    fun `updateProductStatus - product found, should update status, should return product`() {
        val productUpdate = fakeProductStatusUpdateDTO(ProductStatus.SOLD)
        val product = fakeProduct()
        val updatedProduct = product.apply {
            status = ProductStatus.SOLD
        }

        `when`(productRepository.findById(productUpdate.id)).thenReturn(Optional.of(product))
        `when`(productRepository.save(ArgumentMatchers.any(Product::class.java))).thenReturn(updatedProduct)

        val mvcResult = mockMvc.perform(
                post("/products/update/status")
                        .contentType(MediaType.APPLICATION_JSON_VALUE)
                        .content(objectMapper.writeValueAsString(productUpdate)))
                .andExpect(status().isOk)
                .andReturn()

        val responseString = mvcResult.response.contentAsString
        val resultProduct = objectMapper.readValue(responseString, Product::class.java)
        Assert.assertEquals(ProductStatus.SOLD, resultProduct.status)
    }
}